use anyhow::Result;
use core::time;
use defs::{Action, Frame, Mapping, Trigger};
use env_logger::Builder;
use global_hotkey::HotKeyState;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use log::{info, trace, warn};
use serde::{Deserialize, Serialize};
use serialport::{SerialPort, SerialPortInfo, SerialPortType, UsbPortInfo};
use std::collections::HashMap;
use std::io::{self, Write as _};
use std::sync::Mutex;
use std::sync::{mpsc, Arc};
use std::thread::{self, sleep, JoinHandle};
use std::time::Duration;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{DispatchMessageW, GetMessageW, TranslateMessage, HWND_TOP, MSG};

fn serial_port() -> Result<(Box<dyn SerialPort>, Box<dyn SerialPort>)> {
    let ports = serialport::available_ports()?;
    let ports: Vec<SerialPortInfo> = ports
        .into_iter()
        .filter_map(|p| {
            if let SerialPortType::UsbPort(info) = &p.port_type {
                if info.vid == 1027 && info.pid == 24577 {
                    Some(p) // Return the matching port
                } else {
                    None // Ignore other ports
                }
            } else {
                None // Ignore non-USB ports
            }
        })
        .collect();
    let unique_port = ports.get(0).unwrap();
    let port = serialport::new(unique_port.port_name.clone(), 115_200)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .flow_control(serialport::FlowControl::None)
        .timeout(Duration::from_millis(10))
        .open()?;
    let clone = port.try_clone()?;
    Ok((port, clone))
}

fn main() -> Result<()> {
    Builder::new()
        .filter_level(log::LevelFilter::Trace) // Set the minimum level to Info
        .init(); // Initialize the logger
    info!("starting up");

    let (uart_rx, uart_tx) = serial_port()?;

    // let (tx, rx) = std::sync::mpsc::channel::<Command>();
    let (mapping_tx, mapping_rx) = mpsc::channel::<Mapping>();

    let uart_tx0 = Arc::new(Mutex::new(uart_tx));
    let uart_tx1 = uart_tx0.clone();

    // listen and send mapping
    thread::spawn(move || loop {
        match mapping_rx.recv() {
            Ok(mapping) => {
                let s = bincode::serialize(&Frame::Mapping(mapping)).unwrap();
                uart_tx0.lock().unwrap().write(&s).unwrap();
            }
            Err(e) => {
                warn!("{}", e);
            }
        }
    });

    // notify a new mapping
    thread::spawn(move || loop {
        let mut mapping = Mapping::new();
        mapping.add_mapping(
            Trigger {
                modifiers: Some(Modifiers::ALT),
                code: Code::Digit2,
            },
            Action::Key(None, Code::Digit1),
        );
        mapping_tx.send(mapping).unwrap();
    });

    trace!("create GlobalHotKeyManager");
    let manager = GlobalHotKeyManager::new()?;
    let hotkey = HotKey::new(Some(Modifiers::ALT), Code::Digit2);
    trace!("register {hotkey}");
    manager.register(hotkey)?;
    let hotkey_receiver = GlobalHotKeyEvent::receiver();

    let (hotkey_tx, hotkey_rx) = mpsc::channel();

    // listen keyboard hotkey
    thread::spawn(move || loop {
        if let Ok(event) = hotkey_receiver.recv() {
            trace!("tray event: {event:?}");
            if event.id == hotkey.id {
                if event.state == HotKeyState::Pressed {
                    trace!("press ALT+2");
                    hotkey_tx
                        .send(Trigger {
                            modifiers: Some(Modifiers::ALT),
                            code: Code::Digit2,
                        })
                        .unwrap();
                }
            }
        }
    });

    // send hotkey to UART
    thread::spawn(move || loop {
        if let Ok(event) = hotkey_rx.recv() {
            let s = bincode::serialize(&Frame::Trigger(event)).unwrap();
            trace!("UART TX: {s:?}");
            //uart_tx1.lock().unwrap().write_all(&s).unwrap();
        }
    });

    run_win32_message_loop();
    trace!("teardown");
    //app
    Ok(())
}

// fn initialize_connection() -> Option<TcpStream> {
//     debug!("initialize connection");
//     debug!("try tcp connect to 192.168.1.11:12345");
//     match TcpStream::connect("192.168.1.11:12345") {
//         Ok(stream) => {
//             debug!("tcp connected");
//             Some(stream)
//         }
//         Err(_) => {
//             debug!("tcp connection error");
//             None
//         }
//     }
// }
// Simple message loop using winapi
fn run_win32_message_loop() {
    unsafe {
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, HWND_TOP, 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
