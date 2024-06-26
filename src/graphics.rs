pub type Graphic = [[u8; 8]; 8];

pub const EMPTY: Graphic = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

pub const FULL: Graphic = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

pub const LETTER_A: Graphic = [
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
];

pub const LETTER_B: Graphic = [
    [1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 0, 0, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 0, 1, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 0, 0, 0],
];

pub const LETTER_C: Graphic = [
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
];

pub const LETTER_D: Graphic = [
    [1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 0, 0, 0],
    [1, 1, 1, 1, 1, 0, 0, 0],
];

pub const LETTER_E: Graphic = [
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
];

pub const LETTER_F: Graphic = [
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
];

pub const LETTER_G: Graphic = [
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 1, 1, 1, 0, 0],
    [1, 1, 0, 0, 0, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 0, 0],
];

pub const LETTER_H: Graphic = [
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
];

pub const LETTER_I: Graphic = [
    [1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
];

pub const LETTER_J: Graphic = [
    [1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [1, 0, 1, 1, 0, 0, 0, 0],
    [0, 1, 1, 1, 0, 0, 0, 0],
];

pub const LETTER_K: Graphic = [
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 1, 1, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 1, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
];

pub const LETTER_L: Graphic = [
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
];

pub const LETTER_M: Graphic = [
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
];

pub const LETTER_N: Graphic = [
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 1, 0, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 0, 1, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
];

pub const LETTER_O: Graphic = [
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
];

pub const LETTER_P: Graphic = [
    [1, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 1, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
];

pub const LETTER_Q: Graphic = [
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 1, 0, 1, 1, 0, 0],
    [1, 1, 0, 1, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 1, 1, 0, 1, 1, 0],
];

pub const LETTER_R: Graphic = [
    [1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 1, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
];

pub const LETTER_S: Graphic = [
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
];

pub const LETTER_T: Graphic = [
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
];

pub const LETTER_U: Graphic = [
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
];

pub const LETTER_V: Graphic = [
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [0, 1, 0, 0, 1, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
];

pub const LETTER_W: Graphic = [
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
];

pub const LETTER_X: Graphic = [
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [0, 1, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
];

pub const LETTER_Y: Graphic = [
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
];

pub const LETTER_Z: Graphic = [
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 1, 1, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
];

pub const DIGIT_0: Graphic = [
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
];

pub const DIGIT_1: Graphic = [
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
];

pub const DIGIT_2: Graphic = [
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 1, 1, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
];

pub const DIGIT_3: Graphic = [
    [0, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 1, 1, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [1, 0, 0, 0, 1, 1, 0, 0],
    [1, 1, 1, 0, 0, 1, 0, 0],
    [0, 0, 1, 1, 1, 0, 0, 0],
];

pub const DIGIT_4: Graphic = [
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 1, 1, 1, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 0, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
];

pub const DIGIT_5: Graphic = [
    [0, 1, 1, 1, 1, 1, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 0, 0, 0],
    [0, 1, 0, 0, 1, 1, 0, 0],
    [0, 0, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
];

pub const DIGIT_6: Graphic = [
    [0, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 0, 1, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 0, 1, 1, 1, 0, 0, 0],
    [1, 1, 1, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 0, 1, 0, 0],
    [0, 1, 1, 0, 1, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
];

pub const DIGIT_7: Graphic = [
    [0, 1, 1, 1, 1, 1, 0, 0],
    [1, 0, 0, 0, 1, 1, 0, 0],
    [1, 0, 0, 0, 1, 1, 0, 0],
    [0, 0, 0, 0, 1, 1, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
];

pub const DIGIT_8: Graphic = [
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [0, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
];

pub const DIGIT_9: Graphic = [
    [0, 0, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 1, 1, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
];

pub const QUESTION_MARK: Graphic = [
    [0, 0, 1, 1, 1, 0, 0, 0],
    [0, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
];

pub const COLON: Graphic = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

pub fn from_char(c: char) -> &'static Graphic {
    match c.to_ascii_uppercase() {
        'A' => &LETTER_A,
        'B' => &LETTER_B,
        'C' => &LETTER_C,
        'D' => &LETTER_D,
        'E' => &LETTER_E,
        'F' => &LETTER_F,
        'G' => &LETTER_G,
        'H' => &LETTER_H,
        'I' => &LETTER_I,
        'J' => &LETTER_J,
        'K' => &LETTER_K,
        'L' => &LETTER_L,
        'M' => &LETTER_M,
        'N' => &LETTER_N,
        'O' => &LETTER_O,
        'P' => &LETTER_P,
        'Q' => &LETTER_Q,
        'R' => &LETTER_R,
        'S' => &LETTER_S,
        'T' => &LETTER_T,
        'U' => &LETTER_U,
        'V' => &LETTER_V,
        'W' => &LETTER_W,
        'X' => &LETTER_X,
        'Y' => &LETTER_Y,
        'Z' => &LETTER_Z,
        '0' => &DIGIT_0,
        '1' => &DIGIT_1,
        '2' => &DIGIT_2,
        '3' => &DIGIT_3,
        '4' => &DIGIT_4,
        '5' => &DIGIT_5,
        '6' => &DIGIT_6,
        '7' => &DIGIT_7,
        '8' => &DIGIT_8,
        '9' => &DIGIT_9,
        ':' => &COLON,
        ' ' => &EMPTY,
        '\x00' => &EMPTY,
        _ => &QUESTION_MARK,
    }
}

pub const MAX_LEN: usize = 8; // arbitrary
pub struct Panorama {
    pub graphics: [&'static Graphic; MAX_LEN],
    pub len: usize,
}

// pub fn from_str(s: &str) -> Panorama {
//     let mut panorama = Panorama {
//         graphics: [&EMPTY; MAX_LEN],
//         len: s.len(),
//     };
//     let mut chars = s.chars();
//     for i in 0..MAX_LEN {
//         if let Some(c) = chars.next() {
//             panorama.graphics[i] = from_char(c);
//         }
//     }
//     panorama
// }

// // using a macro to handle flexible array length
// macro_rules! text {
//     ($s:expr) => {{
//         let mut chars = $s.chars();
//         let mut arr = [graphics::EMPTY; $s.len()];
//         for i in 0..$s.len() {
//             let c = chars.next().unwrap();
//             let g = graphics::from_char(c);
//             arr[i] = g;
//         }
//         arr
//     }};
// }

// pub(crate) use text;
