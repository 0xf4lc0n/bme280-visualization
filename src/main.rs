extern crate regex;
extern crate serialport;

mod bme280_data;
mod gui;

use bme280_data::Bme280Data;
use gui::draw_graph;
use regex::Regex;
use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;
use std::str;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::thread::spawn;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let (data_from_serial, h1) = receive_data();
    let (parsed_data, h2) = parse_data(data_from_serial);
    draw_graph(parsed_data);

    h1.join().unwrap();
    h2.join().unwrap();
}

fn receive_data() -> (Receiver<String>, JoinHandle<()>) {
    const PORT_NAME: &str = "/dev/ttyACM0";
    const BAUD_RATE: u32 = 38400;

    let serial_port = serialport::new(PORT_NAME, BAUD_RATE)
        .timeout(Duration::from_millis(15))
        .open()
        .expect(&format!("Failed to open {} port!", PORT_NAME));

    println!("Receiving data on {} at {} baud: ", PORT_NAME, BAUD_RATE);

    let mut reader = BufReader::new(serial_port);

    let (sender, receiver) = channel();

    let handle = spawn(move || loop {
        let mut data = String::with_capacity(64);
        match reader.read_line(&mut data) {
            Ok(_) => sender.send(data).unwrap(),
            Err(ref e) if e.kind() == ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    });

    (receiver, handle)
}

fn parse_data(serial_reads: Receiver<String>) -> (Receiver<Bme280Data>, JoinHandle<()>) {
    let re = Regex::new(r"[+-]?\d+\.\d+").unwrap();
    let (sender, receiver) = channel();

    let handle = spawn(move || {
        for data in serial_reads {
            let v: Vec<f32> = re
                .find_iter(&data)
                .map(|m| {
                    m.as_str()
                        .parse::<f32>()
                        .expect("BME280 sends invalid numeric data!")
                })
                .collect();

            if v.len() == 3 {
                let bme_data = Bme280Data::new(v[0], v[1], v[2]);
                sender.send(bme_data).unwrap();
            }
        }
    });

    (receiver, handle)
}
