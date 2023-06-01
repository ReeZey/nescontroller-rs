import net from 'net';
import ViGEmClient from 'vigemclient';

let client = new ViGEmClient();
client.connect();

let controller = client.createX360Controller();
controller.connect();
controller.resetInputs();

/*
A = 1 (B) nintendo likes to swap these
B = 2 (A) nintendo likes to swap these
SELECT = 3
START = 4
UP = 5
DOWN = 6
LEFT = 7
RIGHT = 8
*/

const server = net.createServer((socket) => {
    console.log("connected");
    socket.on('data', (data) => {
    //console.log(data);
    for(let command of Array.from(data)) {
        //console.log(command);

        controller.button.B.setValue(check_bit(command, 1));
        controller.button.A.setValue(check_bit(command, 2));

        controller.button.BACK.setValue(check_bit(command, 4));
        //controller.button.START.setValue(check_bit(command, 8));
        
        let joy_y = 0;
        let joy_x = 0;
        if (check_bit(command, 16)) {
            joy_y += 1;
        }
        if (check_bit(command, 32)) {
            joy_y -= 1;
        }
        
        if (check_bit(command, 64)) {
            joy_x -= 1;
        }
        if (check_bit(command, 128)) {
            joy_x += 1;
        }

        //holding start swaps to use right joystick, instead of left
        if(check_bit(command, 8)) {
            controller.axis.rightX.setValue(joy_x);
            controller.axis.rightY.setValue(joy_y);
        }else {
            controller.axis.leftX.setValue(joy_x);
            controller.axis.leftY.setValue(joy_y);

            controller.axis.rightX.setValue(0);
            controller.axis.rightY.setValue(0);
        }
        
    }
  });

  socket.on('close', () => {
    process.exit(0);
  })
});

function check_bit(command, bit){
    return (command & bit) != 0;
}

server.listen(5050);