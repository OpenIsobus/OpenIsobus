use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex, atomic::AtomicU64};

use open_isobus::iso_11783_6::objects::{
    DataMask, ObjectId, ObjectRef, PictureGraphic, PictureGraphicFormat, PictureGraphicOptions,
    Point,
};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

use open_isobus::iso_11783_6::renderer::{Canvas, ObjectPoolRenderer};
use open_isobus::iso_11783_6::{Object, ObjectPool};

const VT_SCREEN_SIZE: usize = 480;
const SOFT_KEY_SIZE: usize = 60;
const BPP: usize = 4;

static ISOBUS_FPS: AtomicU64 = AtomicU64::new(0);

pub fn main() -> Result<(), String> {
    unsafe {
        std::env::set_var("RUST_LOG", "debug,open_isobus=info");
    }

    env_logger::builder()
        .format_level(false)
        .format_target(false)
        .init();

    let exit_signal = Arc::new(Mutex::new(false));
    let screen_buffer = Arc::new(Mutex::new(vec![0u8; VT_SCREEN_SIZE * VT_SCREEN_SIZE * BPP]));
    let soft_key_buffer: [Arc<Mutex<Vec<u8>>>; 12] = core::array::from_fn(|_| {
        Arc::new(Mutex::new(vec![0u8; SOFT_KEY_SIZE * SOFT_KEY_SIZE * BPP]))
    });

    let thread_exit_signal = exit_signal.clone();
    let thread_screen_buffer = screen_buffer.clone();
    let thread_soft_key_buffer = soft_key_buffer.clone();
    std::thread::spawn(move || {
        isobus_task(
            thread_exit_signal,
            thread_screen_buffer,
            &thread_soft_key_buffer,
        )
    });

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let timer_subsystem = sdl_context.timer()?;

    let mut fps: usize = 0;
    let mut main_thread: u64 = 0;
    let mut next_fps_update = timer_subsystem.performance_counter();
    let mut next_frame_update = timer_subsystem.performance_counter();

    let mut window = video_subsystem
        .window("", 640, 480)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    window
        .set_minimum_size(640, 480)
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut data_mask_area_texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::ABGR8888,
            VT_SCREEN_SIZE as u32,
            VT_SCREEN_SIZE as u32,
        )
        .map_err(|e| e.to_string())?;

    // let mut soft_key_area_texture = texture_creator
    //     .create_texture_streaming(
    //         PixelFormatEnum::ABGR8888,
    //         SOFT_KEY_SIZE as u32,
    //         SOFT_KEY_SIZE as u32,
    //     )
    //     .map_err(|e| e.to_string())?;

    let mut soft_key_area_textures: [sdl2::render::Texture; 12] = core::array::from_fn(|_| {
        texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ABGR8888,
                SOFT_KEY_SIZE as u32,
                SOFT_KEY_SIZE as u32,
            )
            .unwrap()
    });

    let (window_width, window_height) = canvas.window().size();
    let data_mask_area_size: u32 = u32::min(window_width - 160, window_height);
    let x = (window_width - data_mask_area_size) as i32 / 2;
    let y = (window_height - data_mask_area_size) as i32 / 2;
    let mut data_mask_area = Rect::new(x, y, data_mask_area_size, data_mask_area_size);

    let x = (window_width - SOFT_KEY_SIZE as u32 - 10) as i32;
    let mut soft_key_areas = [
        Rect::new(x, 0 + 10, SOFT_KEY_SIZE as u32, SOFT_KEY_SIZE as u32),
        Rect::new(x, 80 + 10, SOFT_KEY_SIZE as u32, SOFT_KEY_SIZE as u32),
        Rect::new(x, 160 + 10, SOFT_KEY_SIZE as u32, SOFT_KEY_SIZE as u32),
        Rect::new(x, 240 + 10, SOFT_KEY_SIZE as u32, SOFT_KEY_SIZE as u32),
        Rect::new(x, 320 + 10, SOFT_KEY_SIZE as u32, SOFT_KEY_SIZE as u32),
        Rect::new(x, 400 + 10, SOFT_KEY_SIZE as u32, SOFT_KEY_SIZE as u32),
        Rect::new(0 + 10, 0 + 10, SOFT_KEY_SIZE as u32, SOFT_KEY_SIZE as u32),
        Rect::new(0 + 10, 80 + 10, SOFT_KEY_SIZE as u32, SOFT_KEY_SIZE as u32),
        Rect::new(0 + 10, 160 + 10, SOFT_KEY_SIZE as u32, SOFT_KEY_SIZE as u32),
        Rect::new(0 + 10, 240 + 10, SOFT_KEY_SIZE as u32, SOFT_KEY_SIZE as u32),
        Rect::new(0 + 10, 320 + 10, SOFT_KEY_SIZE as u32, SOFT_KEY_SIZE as u32),
        Rect::new(0 + 10, 400 + 10, SOFT_KEY_SIZE as u32, SOFT_KEY_SIZE as u32),
    ];

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::Window { win_event, .. } => match win_event {
                    sdl2::event::WindowEvent::SizeChanged(window_width, window_height) => {
                        let data_mask_area_size: i32 = i32::min(window_width - 160, window_height);
                        let x = (window_width - data_mask_area_size) / 2;
                        let y = (window_height - data_mask_area_size) / 2;
                        data_mask_area =
                            Rect::new(x, y, data_mask_area_size as u32, data_mask_area_size as u32);

                        let x = window_width - SOFT_KEY_SIZE as i32 - 10;
                        for soft_key_area in &mut soft_key_areas[..6] {
                            soft_key_area.set_x(x);
                        }
                    }
                    _ => {}
                },
                _ => {
                    // println!("{event:?}")
                }
            }
        }

        if timer_subsystem.performance_counter() > next_frame_update {
            if let Ok(screen_buffer) = screen_buffer.try_lock() {
                let _ = data_mask_area_texture.update(
                    None,
                    screen_buffer.as_slice(),
                    VT_SCREEN_SIZE * BPP,
                );
                next_frame_update += timer_subsystem.performance_frequency() / 50;
                fps += 1;
            }
            for (i, soft_key_buffer) in soft_key_buffer.iter().enumerate() {
                if let Ok(soft_key_buffer) = soft_key_buffer.try_lock() {
                    let _ = soft_key_area_textures[i].update(
                        None,
                        soft_key_buffer.as_slice(),
                        SOFT_KEY_SIZE * BPP,
                    );
                }
            }

            canvas.clear();
            canvas.copy(&data_mask_area_texture, None, Some(data_mask_area))?;
            for (i, soft_key_area_texture) in soft_key_area_textures.iter().enumerate() {
                canvas.copy(&soft_key_area_texture, None, Some(soft_key_areas[i]))?;
            }
            canvas.present();
        }

        // Log FPS once per second
        if timer_subsystem.performance_counter() > next_fps_update {
            let _ = canvas.window_mut().set_title(
                format!(
                    "FPS: {fps}, MAIN_THREAD: {main_thread}, ISOBUS_THREAD: {}",
                    ISOBUS_FPS.load(Ordering::SeqCst)
                )
                .as_str(),
            );
            next_fps_update += timer_subsystem.performance_frequency();
            fps = 0;
            main_thread = 0;
            ISOBUS_FPS.store(0, Ordering::SeqCst);
        }

        main_thread += 1;
    }

    if let Ok(mut exit_signal) = exit_signal.lock() {
        *exit_signal = true;
    }

    Ok(())
}

fn isobus_task(
    exit_signal: Arc<Mutex<bool>>,
    screen_buffer: Arc<Mutex<Vec<u8>>>,
    soft_key_buffer: &[Arc<Mutex<Vec<u8>>>],
) {
    // let mut object_pool = ObjectPool::new();
    // object_pool.add(Object::DataMask(DataMask {
    //     id: 1000.into(),
    //     background_colour: 13,
    //     soft_key_mask: ObjectId::NULL,
    //     object_refs: vec![ObjectRef {
    //         id: 3000.into(),
    //         offset: Point { x: 20, y: 20 },
    //     }],
    //     // object_refs: Vec::new(),
    //     macro_refs: Vec::new(),
    // }));
    // object_pool.add(Object::Container(Container {
    //     id: 2000.into(),
    //     width: 10,
    //     height: 10,
    //     hidden: false,
    //     object_refs: vec![ObjectRef {
    //         id: 3000.into(),
    //         offset: Point { x: 20, y: 20 },
    //     }],
    //     macro_refs: Vec::new(),
    // }));
    // object_pool.add(Object::PictureGraphic(PictureGraphic {
    //     id: 3000.into(),
    //     width: 10,
    //     actual_width: 10,
    //     actual_height: 10,
    //     format: PictureGraphicFormat::Colour8Bit,
    //     options: PictureGraphicOptions::RunLengthEncoded | PictureGraphicOptions::Transparent,
    //     transparency_colour: 13,
    //     data: vec![
    //         10, 10, 10, 1, 10, 2, 10, 3, 10, 4, 10, 5, 10, 6, 10, 7, 10, 8, 10, 9,
    //     ],
    //     macro_refs: Vec::new(),
    // }));

    let data = std::fs::read("test.iop").unwrap();
    let object_pool = ObjectPool::from_iop(data);
    let mut renderer: ObjectPoolRenderer<'_, 30> = ObjectPoolRenderer::new(&object_pool);

    let soft_key_mask = object_pool
        .data_mask_object_by_id(1000.into())
        .and_then(|o| object_pool.soft_key_mask_object_by_id(o.soft_key_mask))
        .unwrap();

    'running: loop {
        if let Ok(mut screen_buffer) = screen_buffer.try_lock() {
            let mut canvas: Canvas<'_> = Canvas::new(
                &mut screen_buffer,
                VT_SCREEN_SIZE as u32,
                VT_SCREEN_SIZE as u32,
            );

            // renderer.draw_working_set_object(&mut canvas);
            // renderer.draw_active_data_mask_object(&mut canvas);
            renderer.draw_object_by_id(&mut canvas, 1000.into());

            ISOBUS_FPS.fetch_add(1, Ordering::SeqCst);

            // break 'running;
        }

        for (i, &object_id) in soft_key_mask.objects.iter().enumerate() {
            // for (i, soft_key_buffer) in soft_key_buffer[..8].iter().enumerate() {
            if let Ok(mut soft_key_buffer) = soft_key_buffer[i].try_lock() {
                let mut canvas: Canvas<'_> = Canvas::new(
                    &mut soft_key_buffer,
                    SOFT_KEY_SIZE as u32,
                    SOFT_KEY_SIZE as u32,
                );

                renderer.draw_object_by_id(&mut canvas, object_id);

                ISOBUS_FPS.fetch_add(1, Ordering::SeqCst);

                // break 'running;
            }
        }

        if let Ok(exit_signal) = exit_signal.try_lock() {
            if *exit_signal {
                break 'running;
            }
        }
    }
}
