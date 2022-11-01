use std::thread;
use std::time::{Duration, Instant};

use open_isobus::iso_11783_5::Name;
use open_isobus::iso_11783_6::ObjectPool;
use open_isobus::Isobus;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();
    
    // Start the isobus thread
    thread::spawn(|| {
        isobus_task();
    });

    // Start some other thread
    thread::spawn(|| {
        heartbeat_task();
    });

    // For example; Do all of our GUI in the main thread.
    loop {}
}

fn isobus_task() {
    // Build a new Isobus instance.
    let mut isobus = Isobus::builder()
        .name(
            Name::builder()
                .has_self_configurable_address(true) // Dynamicaly claim address
                .industry_group(2) // Agricultural machinery
                .device_class(25) // Slurry/Manure Applicators
                .function(128) // Slurry/Manure Rate Control
                .manufacturer_code(0) // unknown
                .build(),
        )
        .address_to_claim(open_isobus::IsobusAddress(0x80))
        .build();

    // iop file paths
    let iop_file_path_in = std::path::PathBuf::from("input.iop");
    let iop_file_path_out = std::path::PathBuf::from("output.iop");

    // Read iop file.
    let iop_data = match std::fs::read(iop_file_path_in) {
        Ok(f) => f,
        Err(_) => Vec::new(),
    };
    let op: ObjectPool = ObjectPool::from_iop(iop_data);
    let op_data: Vec<u8> = op.as_iop();

    // Write iop file to compare.
    std::fs::write(iop_file_path_out, op_data).unwrap();

    let startup_time = Instant::now();

    // Run the isobus process forever, as fast as possible.
    loop {
        isobus.process(Instant::now().duration_since(startup_time).as_millis() as u64);

        thread::yield_now();
    }
}

fn heartbeat_task() {
    loop {
        log::info!("tick");
        thread::sleep(Duration::from_secs(1));
    }
}
