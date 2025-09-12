/*
https://www.codingame.com/ide/puzzle/mars-lander-episode-1

The goal for your program is to safely land the "Mars Lander" shuttle,
the landing ship which contains the Opportunity rover. Mars Lander is guided by a program,
and right now the failure rate for landing on the NASA simulator is unacceptable.

Note that it may look like a difficult problem, but in reality the problem is easy to solve.
This puzzle is the first level among three,
therefore, we need to present you some controls you won't need in order to complete this first level.
 */
use std::io;
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let surface_n = parse_input!(input_line, i32); // the number of points used to draw the surface of Mars.
    for i in 0..surface_n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let land_x = parse_input!(inputs[0], i32); // X coordinate of a surface point. (0 to 6999)
        let land_y = parse_input!(inputs[1], i32); // Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
    }
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
        // 2 integers: rotate power. rotate is the desired rotation angle (should be 0 for level 1),
        // power is the desired thrust power (0 to 4).
        if v_speed > 30 {
            println!("0 4");
        } else if y < 1500 {
            println!("0 4");
        } else {
            println!("0 3");
        }
    }
}
