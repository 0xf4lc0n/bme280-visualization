extern crate piston_window;
extern crate plotters;

mod backend;

use super::sensor::data::Data;
use backend::draw_piston_window;
use piston_window::{EventLoop, WindowSettings};
use plotters::prelude::*;
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;

use std::time::Duration;

const FPS: u32 = 60;
const CAPTIONS: [&str; 3] = ["Temperature", "Humidity", "Pressure"];

fn draw_content(
	b: backend::PistonBackend,
	data: &mut [VecDeque<f32>; 3],
	parsed_data: &Receiver<Data>,
	time_shift: &mut u32,
) -> Result<(), Box<dyn std::error::Error>> {
	let root = b.into_drawing_area();
	root.fill(&WHITE)?;

	if let Ok(v) = parsed_data.recv_timeout(Duration::from_nanos(1)) {
		data[0].push_back(v.temperature);
		data[1].push_back(v.humidity);
		data[2].push_back(v.pressure);
	}

	let mut cc = ChartBuilder::on(&root)
		.margin(10)
		.caption("Real time BME280 statistics", ("sans-serif", 30))
		.x_label_area_size(50)
		.y_label_area_size(100)
		.build_cartesian_2d(*time_shift..(50 + *time_shift), 0f32..1000f32)?;

	cc.configure_mesh()
		.x_labels(10)
		.y_labels(100)
		.x_desc("Seconds")
		.y_desc("Value")
		.draw()?;

	for (idx, data) in (0..).zip(data.iter()) {
		cc.draw_series(LineSeries::new(
			(0..).zip(data.iter()).map(|(a, b)| (a + *time_shift, *b)),
			&Palette99::pick(idx + 5),
		))?
		.label(format!("{}", CAPTIONS[idx]))
		.legend(move |(x, y)| {
			Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(idx + 5))
		});
	}

	cc.configure_series_labels()
		.background_style(&WHITE.mix(0.8))
		.border_style(&BLACK)
		.label_font(("Arial", 20))
		.draw()?;

	if data[0].len() == 50 {
		data[0].drain(0..25);
		data[1].drain(0..25);
		data[2].drain(0..25);
		*time_shift += 25;
	}

	Ok(())
}

pub fn display_graph(parsed_data: Receiver<Data>) -> () {
	let mut window: piston_window::PistonWindow =
		WindowSettings::new("BME280 measure statistics", [450, 300])
			.samples(4)
			.build()
			.unwrap();

	window.set_max_fps(FPS as u64);

	let mut data = [
		VecDeque::with_capacity(10),
		VecDeque::with_capacity(10),
		VecDeque::with_capacity(10),
	];

	let mut time_shift = 0;

	while let Some(_) = draw_piston_window(&mut window, |b| {
		draw_content(b, &mut data, &parsed_data, &mut time_shift)
	}) {}
}
