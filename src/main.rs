mod gui;
mod sensor;

use gui::display_graph;
use sensor::Sensor;
use std::env::args;

static HELP_MESSAGE: &'static str = "Visualization of BME280 sensor measurements\n\n\
                                     USAGE:\n\
                                     \tbme280_vizualizer [OPTIONS]\n\n\
                                     OPTIONS:\n\
                                     \t-h, --help \t\t Prints help information\n\
                                     \t-p, --port \t\t Specifies port\n\
                                     \t-b, --baudrate \t\t Specifies baudrate\n\n\
                                     EXAMPLES:\n\
                                     \tbme280_vizualizer -p /dev/ttyACM0 -b 38400\n\
                                     \tbme280_vizualizer --port /dev/ttyACM0 --baudrate 38400\n";

fn main() {
    let args: Vec<String> = args().collect();

    match &args[..] {
        [_, hf] => match hf.as_str() {
            "-h" | "--help" => println!("{}", HELP_MESSAGE),
            _ => println!("Invalid usage.\nSee bme280_vizualizer --help"),
        },

        [_, pf, p, bf, b]
            if (pf == "-p" || pf == "--port") && (bf == "-b" || bf == "--baudrate") =>
        {
            match b.parse::<u32>() {
                Ok(baud_rate) => run(p, baud_rate),
                Err(_) => println!("{} is invalid baudrate", b),
            }
        }

        _ => println!("Invalid usage.\nSee bme280_vizualizer --help"),
    }
}

fn run(port: &str, baud_rate: u32) {
    let bme280 = match Sensor::new(port, baud_rate) {
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
