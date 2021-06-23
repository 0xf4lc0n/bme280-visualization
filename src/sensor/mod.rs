extern crate lazy_static;
extern crate serialport;

pub mod data;
mod error;

use data::Data;
use error::Error;
use lazy_static::*;
use regex::Regex;
use serialport::ErrorKind;
use std::boxed::Box;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::thread::spawn;
use std::thread::JoinHandle;
use std::time::Duration;

lazy_static! {
	static ref RE: Regex = Regex::new(r"[+-]?\d+\.\d+").unwrap();
}

type SensorResult<T> = std::result::Result<T, Error>;

pub struct Sensor {
	pub port: Box<dyn serialport::SerialPort>,
}

impl Sensor {
	pub fn new(port_name: &str, baud_rate: u32) -> SensorResult<Self> {
		let serial_port = match serialport::new(port_name, baud_rate)
			.timeout(Duration::from_millis(10))
			.open()
		{
			Ok(port) => port,
			Err(e) if e.kind() == ErrorKind::Io(std::io::ErrorKind::NotFound) => {
				return Err(Error {
					description: format!(
						"Cannot open {} port. There is no such port!",
						port_name
					),
				});
			}
			Err(e) => {
				return Err(Error {
					description: e.description,
				});
			}
		};

		Ok(Sensor { port: serial_port })
	}

	pub fn start_receiving_data(
		sensor: Sensor,
	) -> (Receiver<String>, JoinHandle<SensorResult<()>>) {
		let mut reader = BufReader::new(sensor.port);

		let (sender, receiver) = channel();

		let handle = spawn(move || loop {
			let mut data = String::with_capacity(64);

			match reader.read_line(&mut data) {
				Ok(_) => sender.send(data)?,
				Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
				Err(e) => eprintln!("{:?}", e),
			}
		});

		(receiver, handle)
	}

	pub fn parse_data(
		raw_data: Receiver<String>,
	) -> (Receiver<Data>, JoinHandle<SensorResult<()>>) {
		let (sender, receiver) = channel();

		let handle = spawn(move || {
			for data in raw_data {
				let v: Vec<f32> = RE
					.find_iter(&data)
					.map(|m| {
						m.as_str().parse::<f32>().expect(
							"BME280 sends invalid numeric data!",
						)
					})
					.collect();

				if v.len() == 3 {
					let bme_data = Data::new(v[0], v[1], v[2]);
					sender.send(bme_data)?;
				}
			}

			Ok(())
		});

		(receiver, handle)
	}
}
