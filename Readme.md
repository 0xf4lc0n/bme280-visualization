# BME280-Visualization

![](./bme280-measure.gif)

## Usage

```plain
bme280_vizualizer [OPTIONS]

OPTIONS:
	-h, --help 		 Prints help information
	-p, --port 		 Specifies port
	-b, --baudrate 		 Specifies baudrate

EXAMPLES:
	bme280_vizualizer -p /dev/ttyACM0 -b 38400
	bme281_vizualizer --port /dev/ttyACM0 --baudrate 38400
```

## Misc
[backend.rs](./src/gui/backend.rs) author: https://github.com/plotters-rs/plotters-piston
