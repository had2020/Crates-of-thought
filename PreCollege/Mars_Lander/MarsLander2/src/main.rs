use std::io;

/*
https://www.codingame.com/ide/puzzle/mars-lander-episode-2

The goal for your program is to safely land the "Mars Lander" shuttle,
the landing ship which contains the Opportunity rover. Mars Lander is guided by a program,
and right now the failure rate for landing on the NASA simulator is unacceptable.

This puzzle is the second level of the "Mars Lander" trilogy.
The controls are the same as the previous level but you must now control the angle in order to succeed.
 */

use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut last_point: (i32, i32) = (0, 0);
    let mut target: (i32, i32) = (0, 0);
    let mut was_flat: bool = false;
    let mut require_same: u8 = 100;
    let mut lock_first_target: bool = false;

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let surface_n = parse_input!(input_line, i32); // the number of points used to draw the surface of Mars.

    for i in 0..surface_n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let land_x = parse_input!(inputs[0], i32); // X coordinate of a surface point. (0 to 6999)
        let land_y = parse_input!(inputs[1], i32); // Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
        if !was_flat {
            if land_y == last_point.1 {
                if !lock_first_target {
                    target = (land_x, land_y);
                }
                if require_same == 0 {
                    was_flat = true;
                } else {
                    require_same -= 1;
                }
            } else {
                last_point = (land_x, land_y);
            }
        } else if target != (land_x, land_y) {
            target = ((land_x - target.0), target.1);
            break;
        } else {
            last_point = (land_x, land_y);
        }
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let h_speed = parse_input!(inputs[2], i32); // the horizontal speed (in m/s), can be negative.
        let v_speed = parse_input!(inputs[3], i32); // the vertical speed (in m/s), can be negative.
        let fuel = parse_input!(inputs[4], i32); // the quantity of remaining fuel in liters.
        let rotate = parse_input!(inputs[5], i32); // the rotation angle in degrees (-90 to 90).
        let power = parse_input!(inputs[6], i32); // the thrust power (0 to 4).

        eprintln!("target: {:?}", target);

        if (target.0 - x) < 100 {
        } else {
            if ((target.0 - 100) - x) < 0 {
                // left
                println!("25 3");
            } else {
                //right
                println!("-25 3");
            }
        }

        /*
        if v_speed > 30 {
            println!("0 4");
        } else if y < 1500 {
            println!("0 4");
        } else {
            println!("0 3");
        }
        */
    }
}
