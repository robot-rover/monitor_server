extern crate systemstat;
extern crate serialport;
extern crate byteorder;

use systemstat::{System, Platform, DelayedMeasurement, CPULoad, Duration};
use std::io::{Cursor, Write};
use std::thread;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use serialport::{SerialPortSettings, SerialPort};
use std::io;
use std::ops::DerefMut;

mod protocol;


fn main() {
    let mut settings = SerialPortSettings::default();
    settings.timeout = Duration::from_secs(1);
    let mut port = serialport::open_with_settings("COM3", &settings).expect("Unable to open port");
    loop {
        let load = cpu_measure().unwrap();
        write_cpu((1.0-load.idle) * 100.0, port.deref_mut());
    }
}

fn write_cpu(percent: f32, port: &mut dyn SerialPort) -> io::Result<()> {
    let usage = (percent * 100.0) as u16;
    let mut buff = &mut [0u8; 3];
    {
        let mut cursor = buff as &mut [u8];
        cursor.write_u8(0x05u8)?;
        cursor.write_u16::<BigEndian>(usage)?;
    }

    port.write_u8(0x01)?;
    assert_eq!(port.read_u8()?, 0x01);
    port.write_all(buff)?;
    Ok(())
}

fn cpu_measure() -> io::Result<CPULoad> {
    let sys = System::new();
    let cpu = sys.cpu_load_aggregate()?;
    thread::sleep(Duration::from_millis(500));
    let cpu = cpu.done()?;
    // println!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
    //          cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
    Ok(cpu)
}
