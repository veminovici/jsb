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
const SCALE_PATTERN_MINOR_MASK: u16 = 0b0000_0000_0000_0111;
const SCALE_PATTERN_MINOR_WHOLE_HALF: u16 = 0b0110;
const SCALE_PATTERN_MINOR_HALF_WHOLE: u16 = 0b0101;

const SCALE_PATTERN_MAJOR_MASK: u16 = 0b0000_0000_0000_1111;
const SCALE_PATTERN_MAJOR_WHOLE_WHOLE: u16 = 0b1010;
const SCALE_PATTERN_MAJOR_HALF_WHOLE_AND_HALF: u16 = 0b1001;

pub const MAJOR_SCALE: ScalePattern = ScalePattern::MAJOR;
pub const MAJOR_SCALE_INTERVALS: [i8; 7] = [2, 4, 5, 7, 9, 11, 12];
pub const MAJOR_SCALE_STEPS: [i8; 7] = [2, 2, 1, 2, 2, 2, 1];

pub const NATURAL_MINOR_SCALE: ScalePattern = ScalePattern::NATURAL_MINOR;
pub const NATURAL_MINOR_SCALE_INTERVALS: [i8; 7] = [2, 3, 5, 7, 8, 10, 12];
pub const NATURAL_MINOR_SCALE_STEPS: [i8; 7] = [2, 1, 2, 2, 1, 2, 2];

pub const HARMONIC_MINOR_SCALE: ScalePattern = ScalePattern::HARMONIC_MINOR;
pub const HARMONIC_MINOR_SCALE_INTERVALS: [i8; 7] = [2, 3, 5, 7, 8, 11, 12];
pub const HARMONIC_MINOR_SCALE_STEPS: [i8; 7] = [2, 1, 2, 2, 1, 3, 1];

pub const MELODIC_MINOR_SCALE: ScalePattern = ScalePattern::MELODIC_MINOR;
pub const MELODIC_MINOR_SCALE_INTERVALS: [i8; 7] = [2, 3, 5, 7, 9, 11, 12];
pub const MELODIC_MINOR_SCALE_STEPS: [i8; 7] = [2, 1, 2, 2, 2, 2, 1];

impl ScalePattern {
    /// Get the scale pattern.
    #[inline]
    pub const fn pattern(&self) -> u16 {
        self.bits() & SCALE_PATTERN_MASK
    }

    #[inline]
    pub const fn is_major(&self) -> bool {
        let bits = self.bits() & SCALE_PATTERN_MAJOR_MASK;
        bits == SCALE_PATTERN_MAJOR_WHOLE_WHOLE || bits == SCALE_PATTERN_MAJOR_HALF_WHOLE_AND_HALF
    }

    #[inline]
    pub const fn is_minor(&self) -> bool {
        let bits = self.bits() & SCALE_PATTERN_MINOR_MASK;
        bits == SCALE_PATTERN_MINOR_WHOLE_HALF || bits == SCALE_PATTERN_MINOR_HALF_WHOLE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn get_intervals(pattern: &ScalePattern) -> impl Iterator<Item = i8> {
        let pattern = pattern.pattern();

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

    pub fn get_steps(pattern: &ScalePattern) -> impl Iterator<Item = i8> {
        let mut last = 0;
        get_intervals(pattern).map(move |interval| {
            let step = interval - last;
            last = interval;
            step
        })
    }

    #[test]
    fn test_major_scale_intervals() {
        let intervals = get_intervals(&MAJOR_SCALE).collect::<Vec<_>>();
        assert_eq!(intervals, MAJOR_SCALE_INTERVALS);
    }

    #[test]
    fn test_major_scale_steps() {
        let steps = get_steps(&MAJOR_SCALE).collect::<Vec<_>>();
        assert_eq!(steps, MAJOR_SCALE_STEPS);
    }

    #[test]
    fn test_major_scale_quality() {
        assert!(MAJOR_SCALE.is_major());
        assert!(!MAJOR_SCALE.is_minor());
    }

    #[test]
    fn test_minor_scale_intervals() {
        let intervals = get_intervals(&NATURAL_MINOR_SCALE).collect::<Vec<_>>();
        assert_eq!(intervals, NATURAL_MINOR_SCALE_INTERVALS);
    }

    #[test]
    fn test_minor_scale_steps() {
        let steps = get_steps(&NATURAL_MINOR_SCALE).collect::<Vec<_>>();
        assert_eq!(steps, NATURAL_MINOR_SCALE_STEPS);
    }

    #[test]
    fn test_minor_scale_quality() {
        assert!(!NATURAL_MINOR_SCALE.is_major());
        assert!(NATURAL_MINOR_SCALE.is_minor());
    }

    #[test]
    fn test_harmonic_scale_intervals() {
        let intervals = get_intervals(&HARMONIC_MINOR_SCALE).collect::<Vec<_>>();
        assert_eq!(intervals, HARMONIC_MINOR_SCALE_INTERVALS);
    }

    #[test]
    fn test_harmonic_scale_steps() {
        let steps = get_steps(&HARMONIC_MINOR_SCALE).collect::<Vec<_>>();
        assert_eq!(steps, HARMONIC_MINOR_SCALE_STEPS);
    }

    #[test]
    fn test_harmonic_scale_quality() {
        assert!(!HARMONIC_MINOR_SCALE.is_major());
        assert!(HARMONIC_MINOR_SCALE.is_minor());
    }

    #[test]
    fn test_melodic_scale_intervals() {
        let intervals = get_intervals(&MELODIC_MINOR_SCALE).collect::<Vec<_>>();
        assert_eq!(intervals, MELODIC_MINOR_SCALE_INTERVALS);
    }

    #[test]
    fn test_melodic_scale_steps() {
        let steps = get_steps(&MELODIC_MINOR_SCALE).collect::<Vec<_>>();
        assert_eq!(steps, MELODIC_MINOR_SCALE_STEPS);
    }

    #[test]
    fn test_melodic_scale_quality() {
        assert!(!MELODIC_MINOR_SCALE.is_major());
        assert!(MELODIC_MINOR_SCALE.is_minor());
    }
}
