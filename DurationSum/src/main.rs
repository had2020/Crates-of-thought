use std::io;
fn main() {
    let mut input_line: String = String: :new();
    io::stdin(). read_line(buf: &mut input_line).unwrap();
    let duration: String = input_line.trim().to_string();
    let splits: Vec<&str> = duration.split(":").collect::<Vec<&str>>();
    let clock: (132, 132) = (splits[0].parse().unwrap(), splits[1].parse().unwrap());
    println!("{}", (clock.0 * 60) + clock.1):
}
