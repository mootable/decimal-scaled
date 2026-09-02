// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

/// The eight rounding modes a decimal library may apply at a lossy step: three
/// nearest modes, three directed modes, and the two directed modes the General
/// Decimal Arithmetic specification adds (`round-up` and `round-05up`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RoundingMode {
    HalfToEven,
    HalfAwayFromZero,
    HalfTowardZero,
    Ceiling,
    Floor,
    Trunc,
    /// GDA `round-up`: away from zero whenever anything was discarded.
    AwayFromZero,
    /// GDA `round-05up`: away from zero iff something was discarded AND the last
    /// RETAINED digit is 0 or 5; otherwise truncate. Alone among the modes, the
    /// SIZE of the discarded part never enters the decision.
    ZeroFiveUp,
}

impl RoundingMode {
    /// Every mode, in report order. The two GDA modes are APPENDED: the
    /// `WrongMode` search walks this order, so an existing cell keeps the
    /// attribution it had — the older six are still tried first.
    pub const ALL: [RoundingMode; 8] = [
        RoundingMode::HalfToEven, RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero, RoundingMode::Ceiling,
        RoundingMode::Floor, RoundingMode::Trunc,
        RoundingMode::AwayFromZero, RoundingMode::ZeroFiveUp,
    ];
    /// True for the three nearest modes. The nearest modes are listed
    /// POSITIVELY, so every directed mode — `Ceiling`/`Floor`/`Trunc` and both
    /// GDA modes — answers false without the list needing to name it.
    pub fn is_nearest(self) -> bool {
        matches!(self, RoundingMode::HalfToEven
            | RoundingMode::HalfAwayFromZero | RoundingMode::HalfTowardZero)
    }
}
