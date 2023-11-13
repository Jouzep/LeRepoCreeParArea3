use std::fmt;

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.array)
    }
}
#[derive(Debug)]
#[derive(Clone)]
pub struct Key {
    pub array: [[u8; 4]; 4],
}

fn euclidean_division(dividend: &u8, divisor: u8) -> (u8, u8) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}
impl Key {
    pub fn new(key : String) -> Self {
        let mut two_char_array: Vec<String> = Vec::new();
        for chunk in key.chars().collect::<Vec<_>>().chunks(2) {
            let two_chars: String = chunk.iter().collect();
            two_char_array.push(two_chars);
        }
        let decimal_values: Vec<u8> = two_char_array
            .iter()
            .map(|hex_str| u8::from_str_radix(hex_str, 16).unwrap_or_default())
            .collect();

        let double_array: [[u8; 4]; 4] = {
            let mut iter = decimal_values.iter();
            let mut array = [[0u8; 4]; 4];

            for row in array.iter_mut() {
                for elem in row.iter_mut() {
                    *elem = *iter.next().unwrap_or(&0);
                }
            }
            array
        };
        Key {
            array: double_array,
        }
    }

    pub fn new_w_s_box(key : &Key, rcon_index: usize) -> Self {
        let mut test: [[u8; 4]; 4] = [[0; 4]; 4];

        for index1 in 0.. test.len() {
            let w4 = key.array[index1]; // 4 - 4  w-4 for xor
            if index1 == 0 {
                let mut w1 = key.array[3]; // 4 - 1  w-1
                w1.rotate_left(1); // Rotate
                for (index2, element2) in w1.iter_mut().enumerate() {
                    if index2 == 0 {
                        *element2 = sub_bytes(element2) ^ w4[index2] ^ RCON[rcon_index];
                    }
                    else {
                        *element2 = sub_bytes(element2) ^ w4[index2];
                    }
                }
                test[index1] = w1;
            } else {
                let mut w1 = test[index1 - 1]; // 4 - 1  w-1
                for (index2, element2) in w1.iter_mut().enumerate() {
                    *element2 = *element2 ^ w4[index2];
                }
                test[index1] = w1;
            }
        }
        Key {
            array: test
        }
    }
}

pub fn sub_bytes(number: &u8) -> u8 {
    let (first, second) = euclidean_division(number, 16);
    return S_BOX[first as usize][second as usize]
}
pub const S_BOX: [[u8; 16]; 16] = [
[0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b , 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76],
[0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0],
[0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15],
[0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75],
[0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84],
[0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf],
[0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8],
[0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2],
[0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73],
[0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb],
[0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79],
[0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08],
[0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a],
[0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e],
    [0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf],
    [0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16],
];

pub const RCON: [u8; 10] = [
    0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36,
];

