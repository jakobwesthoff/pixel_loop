// Kudos to Mike Swan (n00dles) for creating this "tetromino font" in the first
// place:
// https://github.com/n00dles/esp_p10_tetris_clock/blob/master/src/numbers.h

use crate::tetromino::AnimStep;

pub const ZERO: &'static [AnimStep] = &[
    AnimStep::from_numeric(2, 5, 4, 16, 0),
    AnimStep::from_numeric(4, 7, 2, 16, 1),
    AnimStep::from_numeric(3, 4, 0, 16, 1),
    AnimStep::from_numeric(6, 6, 1, 16, 1),
    AnimStep::from_numeric(5, 1, 4, 14, 0),
    AnimStep::from_numeric(6, 6, 0, 13, 3),
    AnimStep::from_numeric(5, 1, 4, 12, 0),
    AnimStep::from_numeric(5, 1, 0, 11, 0),
    AnimStep::from_numeric(6, 6, 4, 10, 1),
    AnimStep::from_numeric(6, 6, 0, 9, 1),
    AnimStep::from_numeric(5, 1, 1, 8, 1),
    AnimStep::from_numeric(2, 5, 3, 8, 3),
];

pub const ONE: &'static [AnimStep] = &[
    AnimStep::from_numeric(2, 5, 4, 16, 0),
    AnimStep::from_numeric(3, 4, 4, 15, 1),
    AnimStep::from_numeric(3, 4, 5, 13, 3),
    AnimStep::from_numeric(2, 5, 4, 11, 2),
    AnimStep::from_numeric(0, 0, 4, 8, 0),
];

pub const TWO: &'static [AnimStep] = &[
    AnimStep::from_numeric(0, 0, 4, 16, 0),
    AnimStep::from_numeric(3, 4, 0, 16, 1),
    AnimStep::from_numeric(1, 2, 1, 16, 3),
    AnimStep::from_numeric(1, 2, 1, 15, 0),
    AnimStep::from_numeric(3, 4, 1, 12, 2),
    AnimStep::from_numeric(1, 2, 0, 12, 1),
    AnimStep::from_numeric(2, 5, 3, 12, 3),
    AnimStep::from_numeric(0, 0, 4, 10, 0),
    AnimStep::from_numeric(3, 4, 1, 8, 0),
    AnimStep::from_numeric(2, 5, 3, 8, 3),
    AnimStep::from_numeric(1, 2, 0, 8, 1),
];

pub const THREE: &'static [AnimStep] = &[
    AnimStep::from_numeric(1, 2, 3, 16, 3),
    AnimStep::from_numeric(2, 5, 0, 16, 1),
    AnimStep::from_numeric(3, 4, 1, 15, 2),
    AnimStep::from_numeric(0, 0, 4, 14, 0),
    AnimStep::from_numeric(3, 4, 1, 12, 2),
    AnimStep::from_numeric(1, 2, 0, 12, 1),
    AnimStep::from_numeric(3, 4, 5, 12, 3),
    AnimStep::from_numeric(2, 5, 3, 11, 0),
    AnimStep::from_numeric(3, 4, 1, 8, 0),
    AnimStep::from_numeric(1, 2, 0, 8, 1),
    AnimStep::from_numeric(2, 5, 3, 8, 3),
];

pub const FOUR: &'static [AnimStep] = &[
    AnimStep::from_numeric(0, 0, 4, 16, 0),
    AnimStep::from_numeric(0, 0, 4, 14, 0),
    AnimStep::from_numeric(3, 4, 1, 12, 0),
    AnimStep::from_numeric(1, 2, 0, 12, 1),
    AnimStep::from_numeric(2, 5, 0, 10, 0),
    AnimStep::from_numeric(2, 5, 3, 12, 3),
    AnimStep::from_numeric(3, 4, 4, 10, 3),
    AnimStep::from_numeric(2, 5, 0, 9, 2),
    AnimStep::from_numeric(3, 4, 5, 10, 1),
];

pub const FIVE: &'static [AnimStep] = &[
    AnimStep::from_numeric(0, 0, 0, 16, 0),
    AnimStep::from_numeric(2, 5, 2, 16, 1),
    AnimStep::from_numeric(2, 5, 3, 15, 0),
    AnimStep::from_numeric(3, 4, 5, 16, 1),
    AnimStep::from_numeric(3, 4, 1, 12, 0),
    AnimStep::from_numeric(1, 2, 0, 12, 1),
    AnimStep::from_numeric(2, 5, 3, 12, 3),
    AnimStep::from_numeric(0, 0, 0, 10, 0),
    AnimStep::from_numeric(3, 4, 1, 8, 2),
    AnimStep::from_numeric(1, 2, 0, 8, 1),
    AnimStep::from_numeric(2, 5, 3, 8, 3),
];

pub const SIX: &'static [AnimStep] = &[
    AnimStep::from_numeric(2, 5, 0, 16, 1),
    AnimStep::from_numeric(5, 1, 2, 16, 1),
    AnimStep::from_numeric(6, 6, 0, 15, 3),
    AnimStep::from_numeric(6, 6, 4, 16, 3),
    AnimStep::from_numeric(5, 1, 4, 14, 0),
    AnimStep::from_numeric(3, 4, 1, 12, 2),
    AnimStep::from_numeric(2, 5, 0, 13, 2),
    AnimStep::from_numeric(3, 4, 2, 11, 0),
    AnimStep::from_numeric(0, 0, 0, 10, 0),
    AnimStep::from_numeric(3, 4, 1, 8, 0),
    AnimStep::from_numeric(1, 2, 0, 8, 1),
    AnimStep::from_numeric(2, 5, 3, 8, 3),
];

pub const SEVEN: &'static [AnimStep] = &[
    AnimStep::from_numeric(0, 0, 4, 16, 0),
    AnimStep::from_numeric(1, 2, 4, 14, 0),
    AnimStep::from_numeric(3, 4, 5, 13, 1),
    AnimStep::from_numeric(2, 5, 4, 11, 2),
    AnimStep::from_numeric(3, 4, 1, 8, 2),
    AnimStep::from_numeric(2, 5, 3, 8, 3),
    AnimStep::from_numeric(1, 2, 0, 8, 1),
];

pub const EIGHT: &'static [AnimStep] = &[
    AnimStep::from_numeric(3, 4, 1, 16, 0),
    AnimStep::from_numeric(6, 6, 0, 16, 1),
    AnimStep::from_numeric(3, 4, 5, 16, 1),
    AnimStep::from_numeric(1, 2, 2, 15, 3),
    AnimStep::from_numeric(4, 7, 0, 14, 0),
    AnimStep::from_numeric(1, 2, 1, 12, 3),
    AnimStep::from_numeric(6, 6, 4, 13, 1),
    AnimStep::from_numeric(2, 5, 0, 11, 1),
    AnimStep::from_numeric(4, 7, 0, 10, 0),
    AnimStep::from_numeric(4, 7, 4, 11, 0),
    AnimStep::from_numeric(5, 1, 0, 8, 1),
    AnimStep::from_numeric(5, 1, 2, 8, 1),
    AnimStep::from_numeric(1, 2, 4, 9, 2),
];

pub const NINE: &'static [AnimStep] = &[
    AnimStep::from_numeric(0, 0, 0, 16, 0),
    AnimStep::from_numeric(3, 4, 2, 16, 0),
    AnimStep::from_numeric(1, 2, 2, 15, 3),
    AnimStep::from_numeric(1, 2, 4, 15, 2),
    AnimStep::from_numeric(3, 4, 1, 12, 2),
    AnimStep::from_numeric(3, 4, 5, 12, 3),
    AnimStep::from_numeric(5, 1, 0, 12, 0),
    AnimStep::from_numeric(1, 2, 2, 11, 3),
    AnimStep::from_numeric(5, 1, 4, 9, 0),
    AnimStep::from_numeric(6, 6, 0, 10, 1),
    AnimStep::from_numeric(5, 1, 0, 8, 1),
    AnimStep::from_numeric(6, 6, 2, 8, 2),
];

fn from_digit(digit: u8) -> &'static [AnimStep] {
    match digit {
        0 => ZERO,
        1 => ONE,
        2 => TWO,
        3 => THREE,
        4 => FOUR,
        5 => FIVE,
        6 => SIX,
        7 => SEVEN,
        8 => EIGHT,
        9 => NINE,
        _ => panic!("Invalid digit {digit} requested."),
    }
}
