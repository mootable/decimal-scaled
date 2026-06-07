/// The six rounding modes a decimal library may apply at a lossy step.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RoundingMode {
    HalfToEven,
    HalfAwayFromZero,
    HalfTowardZero,
    Ceiling,
    Floor,
    Trunc,
}

impl RoundingMode {
    pub const ALL: [RoundingMode; 6] = [
        RoundingMode::HalfToEven, RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero, RoundingMode::Ceiling,
        RoundingMode::Floor, RoundingMode::Trunc,
    ];
    pub fn is_nearest(self) -> bool {
        matches!(self, RoundingMode::HalfToEven
            | RoundingMode::HalfAwayFromZero | RoundingMode::HalfTowardZero)
    }
}
