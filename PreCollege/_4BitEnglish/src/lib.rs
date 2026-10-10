#[cfg(feature = "std")]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TwoString {
    pub vec: Vec<u8>,
}

#[cfg(feature = "std")]
impl TwoString {
    pub fn new() -> Self {
        TwoString { vec: Vec::new() }
    }

    pub fn encode(content: &str) -> Self {
        let mut full_chars = content.len();

        let mut content_chars = content.chars();

        if content.len() % 2 != 0 {
            full_chars = (full_chars + 1) / 2;
        } else {
            full_chars /= 2;
        }

        let mut r = TwoString {
            vec: Vec::with_capacity(full_chars),
        };

        for i in 0..full_chars {
            let l1: u8 = match content_chars.nth(i - 1).unwrap().to_ascii_uppercase() {
                // normal ids
                'A' => 0,
                'E' => 1,
                'I' => 2,
                'O' => 3,
                'U' => 4,
                'T' => 5,
                'N' => 6,
                'S' => 7,
                'H' => 8,
                'R' => 9,
                'D' => 10,
                'L' => 11,
                'M' => 12,
                'P' => 13,
                'K' => 14,
                // conversions
                _ => 15, // whitespace
            };

            let l2: u8 = match content_chars.nth(i).unwrap().to_ascii_uppercase() {
                'A' => 0,
                'E' => 1,
                'I' => 2,
                'O' => 3,
                'U' => 4,
                'T' => 5,
                'N' => 6,
                'S' => 7,
                'H' => 8,
                'R' => 9,
                'D' => 10,
                'L' => 11,
                'M' => 12,
                'P' => 13,
                'K' => 14,
                _ => 15,
            };

            let mut rb: u8 = l1;

            r.vec.push(rb /* l1 + l2 */);
        }
        r
    }

    pub fn get_char(self, nth: usize) -> TwoChar {
        //nth % 2 == 0 // TODO without branch
        // 4 + (4 * (nth % 2))
        (self.vec[nth] >> 4 + (4 * (nth % 2))) as TwoChar
    }

    pub fn chars_amt(self) -> usize {
        self.vec.len() * 2
    }
}

#[repr(u8)]
enum TwoChar {
    A = 0,
    E = 1,
    I = 2,
    O = 3,
    U = 4,
    T = 5,
    N = 6,
    S = 7,
    H = 8,
    R = 9,
    D = 10,
    L = 11,
    M = 12,
    P = 13,
    K = 14,
    SPACE = 15,
}

pub struct TwoStr {
    pub ptr: usize,
    pub len: usize,
}
