use std::{net::{TcpListener, TcpStream}, io::Read, error::Error};

use vigem_client::XButtons;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:5050").unwrap();

	loop {
		let (stream, _) = listener.accept().unwrap();
		println!("new challanger approaches");

		match handle_controller(stream) {
			Ok(_) => {}
			Err(_) => {
				println!("he doid, NEEEXT");
			}
		}
	}
}

fn handle_controller(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
	let client = vigem_client::Client::connect().unwrap();
	let id = vigem_client::TargetId::XBOX360_WIRED;
	let mut target = vigem_client::Xbox360Wired::new(client, id);

	target.plugin().unwrap();
	target.wait_ready().unwrap();
	let mut gamepad = vigem_client::XGamepad::default();

    let mut buf = [0u8; 1];
	loop {
        stream.read_exact(&mut buf)?;
        let data: u8 = buf[0];
        let mut gamepad_input: u16 = 0;

        if check_bit(data, 0) {
            gamepad_input |= XButtons::A;
        }

		if check_bit(data, 1) {
			gamepad_input |= XButtons::B;
		}

		if check_bit(data, 2) {
			gamepad_input |= XButtons::BACK;
		}

		if check_bit(data, 3) {
			gamepad_input |= XButtons::START;
		}

		if check_bit(data, 4) {
			gamepad_input |= XButtons::UP;
		}

		if check_bit(data, 5) {
			gamepad_input |= XButtons::DOWN;
		}

		if check_bit(data, 6) {
			gamepad_input |= XButtons::LEFT;
		}

		if check_bit(data, 7) {
			gamepad_input |= XButtons::RIGHT;
		}

        gamepad.buttons = XButtons(gamepad_input);
        target.update(&gamepad)?;

        //println!("{:08b}", data);
	}
}

fn check_bit(data: u8, value: u8) -> bool {
	return (data & (1 << value)) != 0;
}
