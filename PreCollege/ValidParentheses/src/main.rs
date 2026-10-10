//https://leetcode.com/problems/valid-parentheses/
/*
pub fn is_valid(s: String) -> bool {
    let mut frt_para: u16 = 0; // (
    let mut bck_para: u16 = 0;
    let mut frt_brace: u16 = 0; // {
    let mut bck_brace: u16 = 0;
    let mut frt_bracket: u16 = 0; // [
    let mut bck_bracket: u16 = 0;
    let mut r_flag = s.len() % 2 != 0;
    let s_copy = s.as_bytes();
    for i in s_copy {
        frt_para += ((!(b'(' ^ i) as u16) / 65535);
        bck_para += ((!(b')' ^ i) as u16) / 65535);
        frt_brace += ((!(b'{' ^ i) as u16) / 65535);
        bck_brace += ((!(b'}' ^ i) as u16) / 65535);
        frt_bracket += ((!(b'[' ^ i) as u16) / 65535);
        bck_bracket += ((!(b']' ^ i) as u16) / 65535);

        let closting_err =
            (bck_para > frt_para) || (bck_brace > frt_brace) || (bck_bracket > frt_bracket);

    }
    r_flag
}*/

/*
//https://leetcode.com/problems/valid-parentheses/
pub fn is_valid(s: String) -> bool {
    let mut frt_para: u16 = 0; // (
    let mut bck_para: u16 = 0;
    let mut frt_brace: u16 = 0; // {
    let mut bck_brace: u16 = 0;
    let mut frt_bracket: u16 = 0; // [
    let mut bck_bracket: u16 = 0;
    let mut underflow_occurred: u16 = (s.len() % 2 != 0) as u16;
    let s_bytes = s.as_bytes();
    for &b in s_bytes {
        frt_para += (b == b'(') as u16;
        bck_para += (b == b')') as u16;
        frt_brace += (b == b'{') as u16;
        bck_brace += (b == b'}') as u16;
        frt_bracket += (b == b'[') as u16;
        bck_bracket += (b == b']') as u16;
        let closing_err = ((bck_para > frt_para)
            || (bck_brace > frt_brace)
            || (bck_bracket > frt_bracket)) as u16;
        underflow_occurred |= closing_err;
        // TODO: add a check for cordinates with a bitmap.
    }
    let final_mismatch =
        ((frt_para != bck_para) || (frt_brace != bck_brace) || (frt_bracket != bck_bracket)) as u16;
    (underflow_occurred | final_mismatch) == 0
}*/

pub fn is_valid(s: String) -> bool {
    let bytes = s.as_bytes();
    let len = bytes.len();
    if len % 2 != 0 || len > 64 {
        return false;
    }
    let mut stack: u64 = 0;
    let mut depth: u32 = 0;
    for &b in bytes {
        match b {
            b'(' => {
                stack = (stack << 2) | 0b01;
                depth += 1;
            }
            b'[' => {
                stack = (stack << 2) | 0b10;
                depth += 1;
            }
            b'{' => {
                stack = (stack << 2) | 0b11;
                depth += 1;
            }
            b')' => {
                if depth == 0 || (stack & 0b11) != 0b01 {
                    return false;
                }
                stack >>= 2;
                depth -= 1;
            }
            b']' => {
                if depth == 0 || (stack & 0b11) != 0b10 {
                    return false;
                }
                stack >>= 2;
                depth -= 1;
            }
            b'}' => {
                if depth == 0 || (stack & 0b11) != 0b11 {
                    return false;
                }
                stack >>= 2;
                depth -= 1;
            }
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
    depth == 0
}

fn main() {
    //println!("{}", (2445 / 65535)); // Will be zero
    //println!("{}", (!0_u16 / 65535));
}
