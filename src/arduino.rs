extern crate serial;

use self::serial::prelude::*;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::thread::sleep;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud57600,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone,
};

pub enum Port {
    Open(serial::SystemPort),
    Dummy,
}

pub fn open() -> Port {
    match serial::open("/dev/cu.usbmodem1421") {
        Ok(n) => Port::Open(n),
        Err(e) => {
            println!("Couldn't connect to port");
            Port::Dummy
        },
    }
}

pub fn interact<T: SerialPort>(port: &mut T, receiver: Receiver<String>) -> serial::Result<()> {
    try!(port.configure(&SETTINGS));
    try!(port.set_timeout(Duration::from_secs(5)));
    let mut out_buf: Vec<u8> = vec![0u8; 100]; 

    sleep(Duration::from_millis(3000));

    loop{
        match receiver.recv() {
            Ok(input) =>{
                let buf: Vec<u8> = input.as_bytes().to_vec();
                try!(port.write(&buf[..]));
                //println!("Sending to arduino: {}", input);
                //sleep(Duration::from_millis(1000));
            },
            Err(error) => println!("error: {}", error),
        }

    }
    Ok(())
}

pub fn light_to_msg(colour: & ::render::Colour<i16>, light: i16) -> String{
    format!( "l{}{}x", light, colour)
}
