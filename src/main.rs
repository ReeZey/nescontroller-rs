use wiringpi::pin::Value::{High, Low};
use std::{thread, time::Duration, net::TcpStream, io::Write};

fn main() {
    let mut stream = TcpStream::connect("10.0.0.2:5050").unwrap();

    let pi = wiringpi::setup();

    let data_pin = pi.input_pin(0);
    let clock_pin = pi.output_pin(2);
    let latch_pin = pi.output_pin(3);
    clock_pin.digital_write(Low);
    latch_pin.digital_write(Low);

    let mut last_data: u8 = 0;
    loop {
        latch_pin.digital_write(High);
        latch_pin.digital_write(Low);

	    let mut data: u8 = 0;
        for bit in 0..8 {
	    let bit_value = data_pin.digital_read();
            if bit_value == Low {
                data |= 1 << bit;
            }

            clock_pin.digital_write(Low);
            clock_pin.digital_write(High);

            thread::sleep(Duration::from_micros(2))
        }
        
        if data == 255 {
            continue;
        }

        if last_data == data {
            stream.write(&[data]).unwrap();
        }
        
	    last_data = data;
        thread::sleep(Duration::from_millis(20));
    }
}
