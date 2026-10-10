pub fn reverse_string(s: &mut Vec<char>) {
    /*
    let length = s.len() - 1;
    for i in 0..length {
        /*
        s[i] = (s[i] as u8 ^ s[(length - 1) - i] as u8) as char;
        s[length - 1] = (s[i] as u8 ^ s[(length - 1) - i] as u8) as char;
        s[i] = (s[i] as u8 ^ s[(length - 1) - i] as u8) as char;
        */
        /*
        let sw0 = s[i];
        let sw1 = s[(length - 1) - i];
        println!("sw1: {}, sw0: {}", sw1, sw0);
        s[i] = sw1;
        s[(length - 1) - i] = sw0;
        */
        let sw0 = s[i];
        let sw1 = s[length - i];
        s[i] = sw1;
        s[length - i] = sw0;
    }
    */
    s.reverse();
}
fn main() {
    let mut r = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut r);
    println!("r: {:?}", r);
}
