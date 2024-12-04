use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::Write as _,
    iter::Map,
    sync::{atomic::AtomicBool, mpsc, Arc, Mutex},
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

use anyhow::Result;
use defs::{Action, Frame, Mapping};
use env_logger::Builder;
use log::trace;
use once_cell::sync::Lazy;
use rppal::uart::{Parity, Uart};

static MAPPING: Lazy<Mutex<Mapping>> = Lazy::new(|| Mutex::new(Mapping::new()));
const HID_FILENAME: &str = "/dev/hidg0";

static RELEASE: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

fn press_and_release(inp_rpt: &[u8]) -> Result<()> {
    // // Open /dev/hidg0 for write access
    // let mut out_file = OpenOptions::new()
    //     .write(true) // Open the file with write access
    //     .open(HID_FILENAME)?;

    // // Write the input report to the device
    // out_file.write_all(inp_rpt)?;

    // // If release is true, send the RELEASE byte
    // out_file.write_all(RELEASE)?;
    Ok(())
}

fn main() -> Result<()> {
    Builder::new()
        .filter_level(log::LevelFilter::Trace) // Set the minimum level to Info
        .init(); // Initialize the logger
    trace!("start up");

    // let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;
    // uart.set_read_mode(1, Duration::default());
    let mut port = serialport::new("/dev/serial0", 115_200)
        .timeout(Duration::from_millis(10))
        .open()?;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        let mut len_bytes = [0u8;2];
        trace!("read ...");
        // uart.read(&mut buffer).unwrap();
        port.read_exact(&mut len_bytes).unwrap();
        let len = u16::from_le_bytes(len_bytes);
        trace!("read len: {len}");
        let mut buffer = Vec::with_capacity(len as usize);
        trace!("read ...");
        port.read_exact(&mut buffer).unwrap();
        let frame = bincode::deserialize::<Frame>(&buffer).unwrap();
        match frame {
            Frame::Trigger(trigger) => {
                trace!("trigger: {:?}", trigger);
                tx.send(trigger).unwrap();
            }
            Frame::Mapping(mapping) => {
                trace!("mapping: {:?}", mapping);
                *MAPPING.lock().unwrap() = mapping;
            }
        }
    });

    thread::spawn(move || {
        let mut running: HashMap<Action, (Arc<AtomicBool>, JoinHandle<_>)> = HashMap::new();
        loop {
            let trigger = rx.recv().unwrap();
            let mapping = MAPPING.lock().unwrap();
            let action = mapping.mappings.get(&trigger).unwrap().clone();
            trace!("trigger: {:?}, action: {:?}", trigger, action);
            match action {
                defs::Action::Key(modifiers, code) => {
                    // if Key is running, stop it
                    // if Key is not running, repeat write "A" to /dev/hidg0
                    // There may be many Keys are in running
                    if let Some((atomic, _)) = running.get(&action) {
                        atomic.store(false, std::sync::atomic::Ordering::SeqCst);
                        running.remove(&action);
                    } else {
                        let atomic = Arc::new(AtomicBool::new(true));
                        let atomic0 = atomic.clone();
                        let handle = thread::spawn(move || {
                            while atomic0.load(std::sync::atomic::Ordering::SeqCst) {
                                press_and_release(&[
                                    if let Some(modifier) = modifiers {
                                        defs::modifier2keycode(&modifier)
                                    } else {
                                        0x00
                                    },
                                    0x00,
                                    defs::code2keycode(&code),
                                    0x00,
                                    0x00,
                                    0x00,
                                    0x00,
                                    0x00,
                                ])
                                .unwrap();
                                sleep(Duration::from_millis(100));
                            }
                        });
                        running.insert(action, (atomic, handle));
                    }
                }
                defs::Action::Smart => {}
            }
        }
    });

    println!("Hello, world!");
    sleep(Duration::from_secs(1000));
    Ok(())
}
