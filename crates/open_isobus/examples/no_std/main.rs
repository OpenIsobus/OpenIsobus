#![feature(type_alias_impl_trait)]

// To run this example with no_std, you have to provide an alternative source of time.
// #![no_std]
use std::time::Instant;

extern crate alloc;
use alloc::{string::ToString, vec};

use open_isobus::iso_11783_6::objects::*;
use open_isobus::iso_11783_6::EventType;
use open_isobus::iso_11783_6::ObjectPool;
use open_isobus::iso_11783_6::WorkingSet;

fn main() {
    // Programaticaly build the object pool
    let mut op: ObjectPool = ObjectPool::new();
    op.add(Object::WorkingSet(
        open_isobus::iso_11783_6::objects::WorkingSet {
            id: 0.into(),
            background_colour: 0x00,
            selectable: true,
            active_mask: 6.into(),
            object_refs: vec![ObjectRef {
                id: 7.into(),
                offset: Point { x: 10i16, y: 10i16 },
            }],
            macro_refs: vec![],
            language_codes: vec!["nl".to_string()],
        },
    ));
    op.add(Object::DataMask(DataMask {
        id: 6.into(),
        background_colour: 0x0D,
        soft_key_mask: ObjectId::NULL,
        object_refs: vec![
            ObjectRef {
                id: 1.into(),
                offset: Point { x: 40i16, y: 20i16 },
            },
            ObjectRef {
                id: 3.into(),
                offset: Point { x: 50i16, y: 30i16 },
            },
            ObjectRef {
                id: 5.into(),
                offset: Point {
                    x: 100i16,
                    y: 100i16,
                },
            },
        ],
        macro_refs: vec![],
    }));
    op.add(Object::OutputRectangle(OutputRectangle {
        id: 7.into(),
        line_attributes: 2.into(),
        width: 60,
        height: 60,
        line_suppression: 0,
        fill_attributes: ObjectId::NULL,
        macro_refs: vec![],
    }));
    op.add(Object::OutputRectangle(OutputRectangle {
        id: 1.into(),
        line_attributes: 2.into(),
        width: 50,
        height: 25,
        line_suppression: 0,
        fill_attributes: ObjectId::NULL,
        macro_refs: vec![],
    }));
    op.add(Object::OutputLine(OutputLine {
        id: 3.into(),
        line_attributes: 4.into(),
        width: 80,
        height: 60,
        line_direction: 0,
        macro_refs: vec![],
    }));
    op.add(Object::LineAttributes(LineAttributes {
        id: 2.into(),
        line_colour: 1,
        line_width: 1,
        line_art: u16::MAX,
        macro_refs: vec![],
    }));
    op.add(Object::LineAttributes(LineAttributes {
        id: 4.into(),
        line_colour: 0,
        line_width: 1,
        line_art: u16::MAX,
        macro_refs: vec![],
    }));
    op.add(Object::PictureGraphic(PictureGraphic {
        id: 5.into(),
        width: 5,
        actual_width: 5,
        actual_height: 2,
        format: 2,
        options: 0,
        transparency_colour: 255,
        data: vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4],
        macro_refs: vec![],
    }));

    // Create a new working set instance.
    let mut ws = WorkingSet::new(op);
    let startup_time = Instant::now();

    let mut is_active = false;

    // Run the isobus process forever.
    loop {
        // Process the workingset.
        ws.process(Instant::now().duration_since(startup_time).as_millis() as u64);

        // Check for events.
        if let Some(event) = ws.next_event() {
            match event {
                EventType::OnActivate => {
                    if !is_active {
                        log::info!("Screen Activated!");
                        is_active = true;
                    }
                }
                EventType::OnDeactivate => {
                    if is_active {
                        log::info!("Screen Deactivated!");
                        is_active = false;
                    }
                }
                _ => {},
            }
        }
    }
}
