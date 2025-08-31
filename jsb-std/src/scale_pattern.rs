use bitflags::bitflags;

bitflags! {
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ScalePattern: u16 {
        const MASK_HALF = 0b0000_0000_0000_0001;
        const MASK_WHOLE = 0b0000_0000_0000_0010;

        /// The major scale (Ionian mode).
        /// Pattern: W-W-H-W-W-W-H (Whole-Whole-Half-Whole-Whole-Whole-Half)
        const MAJOR = 0b0000_1101_0101_1010;
    }
}


const SCALE_PATTERN_MASK: u16 = 0b0000_1111_1111_1111;
const SCALE_PATTERN_MASKS: [u16; 12] = [
    0b0000_0000_0000_0001,
    0b0000_0000_0000_0010,
    0b0000_0000_0000_0100,
    0b0000_0000_0000_1000,
    0b0000_0000_0001_0000,
    0b0000_0000_0010_0000,
    0b0000_0000_0100_0000,
    0b0000_0000_1000_0000,
    0b0000_0001_0000_0000,
    0b0000_0010_0000_0000,
    0b0000_0100_0000_0000,
    0b0000_1000_0000_0000,
];



pub const MAJOR_SCALE: ScalePattern = ScalePattern::MAJOR;

impl ScalePattern {
    /// Get the scale pattern.
    #[inline]
    pub const fn pattern(&self) -> u16 {
        self.bits() & SCALE_PATTERN_MASK
    }

    pub fn intervals(&self) -> impl Iterator<Item = i8> {
        let pattern = self.pattern();

        SCALE_PATTERN_MASKS.iter().enumerate().filter_map(move |(index, mask)| {
            if pattern & mask != 0 {
                Some((index + 1) as i8)
            } else {
                None
            }
        })
    }

    pub fn steps(&self) -> impl Iterator<Item = i8> {
        let mut last = 0;
        self.intervals().map(move |interval| {
            let step = interval - last;
            last = interval;
            step
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_scale_intervals() {
        let intervals = MAJOR_SCALE.intervals().collect::<Vec<_>>();
        assert_eq!(intervals, vec![2, 4, 5, 7, 9, 11, 12]);
    }

    #[test]
    fn test_major_scale_steps() {
        let steps = MAJOR_SCALE.steps().collect::<Vec<_>>();
        assert_eq!(steps, vec![2, 2, 1, 2, 2, 2, 1]);
    }
}