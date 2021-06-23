use std::fmt;

pub struct Data {
	pub temperature: f32,
	pub pressure: f32,
	pub humidity: f32,
}

impl Data {
	pub fn new(temperature: f32, pressure: f32, humidity: f32) -> Self {
		Data {
			temperature,
			pressure,
			humidity,
		}
	}
}

impl fmt::Display for Data {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{:.2} deg C, {:.2} hPa, {:.2}%",
			self.temperature, self.pressure, self.humidity
		)
	}
}
