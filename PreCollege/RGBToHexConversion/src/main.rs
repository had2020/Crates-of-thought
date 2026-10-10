fn st(x: i32) -> u8 {
    if x > 255 {
        255
    } else if x < 0 {
        0
    } else {
        x as u8
    }
}
fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{:02X}{:02X}{:02X}", st(r), st(g), st(b))
}

/*
The rgb function is incomplete. Complete it so that passing in RGB decimal values will result in a hexadecimal representation being returned. Valid decimal values for RGB are 0 - 255. Any values that fall out of that range must be rounded to the closest valid value.

Note: Your answer should always be 6 characters long, the shorthand with 3 will not work here.

Examples (input --> output):
255, 255, 255 --> "FFFFFF"
255, 255, 300 --> "FFFFFF"
0, 0, 0       --> "000000"
148, 0, 211   --> "9400D3"
 */
