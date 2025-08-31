use bitflags::bitflags;

bitflags! {
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ScalePattern: u16 {
        /// The major scale (Ionian mode).
        /// Pattern: W-W-H-W-W-W-H (Whole-Whole-Half-Whole-Whole-Whole-Half)
        const MAJOR = 0b0000_1101_0101_1010;

        /// The natural minor scale (Aeolian mode)
        /// Pattern: W-H-W-W-H-W-W (Whole-Half-Whole-Whole-Half-Whole-Whole)
        const NATURAL_MINOR = 0b0000_1010_1101_0110;

        /// The harmonic minor scale
        /// Pattern: W-H-W-W-H-WH-H (WH = Whole and a Half)
        const HARMONIC_MINOR = 0b0000_1100_1101_0110;

        /// The melodic minor scale (ascending)
        /// Pattern: W-H-W-W-W-W-H (Whole-Half-Whole-Whole-Whole-Whole-Half)
        const MELODIC_MINOR = 0b0000_1101_0101_0110;
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
pub const NATURAL_MINOR_SCALE: ScalePattern = ScalePattern::NATURAL_MINOR;
pub const HARMONIC_MINOR_SCALE: ScalePattern = ScalePattern::HARMONIC_MINOR;
pub const MELODIC_MINOR_SCALE: ScalePattern = ScalePattern::MELODIC_MINOR;

impl ScalePattern {
    /// Get the scale pattern.
    #[inline]
    pub const fn pattern(&self) -> u16 {
        self.bits() & SCALE_PATTERN_MASK
    }

    pub fn intervals(&self) -> impl Iterator<Item = i8> {
        let pattern = self.pattern();

        SCALE_PATTERN_MASKS
            .iter()
            .enumerate()
            .filter_map(move |(index, mask)| {
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

    #[test]
    fn test_minor_scale_intervals() {
        let intervals = NATURAL_MINOR_SCALE.intervals().collect::<Vec<_>>();
        assert_eq!(intervals, vec![2, 3, 5, 7, 8, 10, 12]);
    }

    #[test]
    fn test_minor_scale_steps() {
        let steps = NATURAL_MINOR_SCALE.steps().collect::<Vec<_>>();
        assert_eq!(steps, vec![2, 1, 2, 2, 1, 2, 2]);
    }

    #[test]
    fn test_harmonic_scale_intervals() {
        let intervals = HARMONIC_MINOR_SCALE.intervals().collect::<Vec<_>>();
        assert_eq!(intervals, vec![2, 3, 5, 7, 8, 11, 12]);
    }

    #[test]
    fn test_harmonic_scale_steps() {
        let steps = HARMONIC_MINOR_SCALE.steps().collect::<Vec<_>>();
        assert_eq!(steps, vec![2, 1, 2, 2, 1, 3, 1]);
    }

    #[test]
    fn test_melodic_scale_intervals() {
        let intervals = MELODIC_MINOR_SCALE.intervals().collect::<Vec<_>>();
        assert_eq!(intervals, vec![2, 3, 5, 7, 9, 11, 12]);
    }

    #[test]
    fn test_melodic_scale_steps() {
        let steps = MELODIC_MINOR_SCALE.steps().collect::<Vec<_>>();
        assert_eq!(steps, vec![2, 1, 2, 2, 2, 2, 1]);
    }
}
