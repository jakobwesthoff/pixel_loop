use crate::tetromino::AnimStep;

pub const COLON: &'static [AnimStep] = &[
    AnimStep::from_numeric(0, 1, 2, 15, 0),
    AnimStep::from_numeric(0, 6, 2, 11, 0),
];
