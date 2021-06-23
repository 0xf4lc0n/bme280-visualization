mod gui;
mod sensor;

use gui::display_graph;
use sensor::Sensor;

fn main() {
    let bme280 = match Sensor::new("/dev/ttyACM0", 38400) {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e.description);
            std::process::exit(1);
        }
    };

    let (raw_data, h1) = Sensor::start_receiving_data(bme280);
    let (parsed_data, h2) = Sensor::parse_data(raw_data);
    display_graph(parsed_data);

    match h1.join().unwrap() {
        Err(e) if !e.description.contains("closed channel") => eprintln!("Serial reader: {}", e),
        _ => (),
    }

    match h2.join().unwrap() {
        Err(e) if !e.description.contains("closed channel") => eprintln!("Data parser: {}", e),
        _ => (),
    }
}
