use bitflags::bitflags;

bitflags! {
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ScalePattern: u16 {}
}

/// The major scale (Ionian mode).
/// Pattern: W-W-H-W-W-W-H (Whole-Whole-Half-Whole-Whole-Whole-Half)
pub const MAJOR_SCALE: ScalePattern = ScalePattern::from_u16(0b0000_1101_0101_1010);

/// The natural minor scale (Aeolian mode)
/// Pattern: W-H-W-W-H-W-W (Whole-Half-Whole-Whole-Half-Whole-Whole)
pub const NATURAL_MINOR_SCALE: ScalePattern = ScalePattern::from_u16(0b0000_1010_1101_0110);

/// The harmonic minor scale
/// Pattern: W-H-W-W-H-WH-H (WH = Whole and a Half)
pub const HARMONIC_MINOR_SCALE: ScalePattern = ScalePattern::from_u16(0b0000_1100_1101_0110);

/// The melodic minor scale (ascending)
/// Pattern: W-H-W-W-W-W-H (Whole-Half-Whole-Whole-Whole-Whole-Half)
pub const MELODIC_MINOR_SCALE: ScalePattern = ScalePattern::from_u16(0b0000_1101_0101_0110);

impl ScalePattern {
    /// Create a scale pattern from a 16-bit integer.
    #[inline]
    pub const fn from_u16(value: u16) -> Self {
        Self::from_bits_retain(value)
    }

    /// Get the scale pattern.
    #[inline]
    const fn pattern(&self) -> u16 {
        const SCALE_PATTERN_MASK: u16 = 0b0000_1111_1111_1111;

        self.bits() & SCALE_PATTERN_MASK
    }

    /// Check if the scale is major.
    #[inline]
    pub const fn is_major(&self) -> bool {
        const SCALE_PATTERN_MAJOR_MASK: u16 = 0b0000_0000_0000_1111;
        const SCALE_PATTERN_MAJOR_WHOLE_WHOLE: u16 = 0b1010;
        const SCALE_PATTERN_MAJOR_HALF_WHOLE_AND_HALF: u16 = 0b1001;

        let bits = self.bits() & SCALE_PATTERN_MAJOR_MASK;
        bits == SCALE_PATTERN_MAJOR_WHOLE_WHOLE || bits == SCALE_PATTERN_MAJOR_HALF_WHOLE_AND_HALF
    }

    /// Check if the scale is minor.
    #[inline]
    pub const fn is_minor(&self) -> bool {
        const SCALE_PATTERN_MINOR_MASK: u16 = 0b0000_0000_0000_0111;
        const SCALE_PATTERN_MINOR_WHOLE_HALF: u16 = 0b0110;
        const SCALE_PATTERN_MINOR_HALF_WHOLE: u16 = 0b0101;

        let bits = self.bits() & SCALE_PATTERN_MINOR_MASK;
        bits == SCALE_PATTERN_MINOR_WHOLE_HALF || bits == SCALE_PATTERN_MINOR_HALF_WHOLE
    }

    /// Get the intervals of the scale.
    #[inline]
    pub fn intervals(&self) -> impl Iterator<Item = u8> {
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

        let pattern = self.pattern();

        SCALE_PATTERN_MASKS
            .iter()
            .enumerate()
            .filter_map(move |(index, mask)| {
                if pattern & mask != 0 {
                    Some((index + 1) as u8)
                } else {
                    None
                }
            })
    }

    /// Get the steps of the scale.
    #[inline]
    pub fn steps(&self) -> impl Iterator<Item = u8> {
        let mut last = 0;
        self.intervals().map(move |interval| {
            let step = interval - last;
            last = interval;
            step
        })
    }

    #[inline]
    pub fn degree(&self, degree: usize) -> u8 {
        self.intervals()
            .nth(degree - 1)
            .expect("The degree {degree} must be within the bounds of the scale")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_scale() {
        const MAJOR_SCALE_INTERVALS: [u8; 7] = [2, 4, 5, 7, 9, 11, 12];
        const MAJOR_SCALE_STEPS: [u8; 7] = [2, 2, 1, 2, 2, 2, 1];

        assert_eq!(MAJOR_SCALE.pattern(), 0b0000_1101_0101_1010);

        let intervals = MAJOR_SCALE.intervals().collect::<Vec<_>>();
        assert_eq!(intervals, MAJOR_SCALE_INTERVALS);

        let steps = MAJOR_SCALE.steps().collect::<Vec<_>>();
        assert_eq!(steps, MAJOR_SCALE_STEPS);

        assert!(MAJOR_SCALE.is_major());
        assert!(!MAJOR_SCALE.is_minor());

        let d3 = MAJOR_SCALE.degree(3);
        assert_eq!(d3, 5);

        let d5 = MAJOR_SCALE.degree(5);
        assert_eq!(d5, 9);
    }

    #[test]
    fn test_minor_scale() {
        const NATURAL_MINOR_SCALE_INTERVALS: [u8; 7] = [2, 3, 5, 7, 8, 10, 12];
        const NATURAL_MINOR_SCALE_STEPS: [u8; 7] = [2, 1, 2, 2, 1, 2, 2];

        assert_eq!(NATURAL_MINOR_SCALE.pattern(), 0b0000_1010_1101_0110);

        let intervals = NATURAL_MINOR_SCALE.intervals().collect::<Vec<_>>();
        assert_eq!(intervals, NATURAL_MINOR_SCALE_INTERVALS);

        let steps = NATURAL_MINOR_SCALE.steps().collect::<Vec<_>>();
        assert_eq!(steps, NATURAL_MINOR_SCALE_STEPS);

        assert!(!NATURAL_MINOR_SCALE.is_major());
        assert!(NATURAL_MINOR_SCALE.is_minor());

        let d3 = NATURAL_MINOR_SCALE.degree(3);
        assert_eq!(d3, 5);

        let d5 = NATURAL_MINOR_SCALE.degree(5);
        assert_eq!(d5, 8);
    }

    #[test]
    fn test_harmonic_scale() {
        const HARMONIC_MINOR_SCALE_INTERVALS: [u8; 7] = [2, 3, 5, 7, 8, 11, 12];
        const HARMONIC_MINOR_SCALE_STEPS: [u8; 7] = [2, 1, 2, 2, 1, 3, 1];

        assert_eq!(HARMONIC_MINOR_SCALE.pattern(), 0b0000_1100_1101_0110);

        let intervals = HARMONIC_MINOR_SCALE.intervals().collect::<Vec<_>>();
        assert_eq!(intervals, HARMONIC_MINOR_SCALE_INTERVALS);

        let steps = HARMONIC_MINOR_SCALE.steps().collect::<Vec<_>>();
        assert_eq!(steps, HARMONIC_MINOR_SCALE_STEPS);

        assert!(!HARMONIC_MINOR_SCALE.is_major());
        assert!(HARMONIC_MINOR_SCALE.is_minor());

        let d3 = HARMONIC_MINOR_SCALE.degree(3);
        assert_eq!(d3, 5);

        let d5 = HARMONIC_MINOR_SCALE.degree(5);
        assert_eq!(d5, 8);
    }

    #[test]
    fn test_melodic_scale() {
        const MELODIC_MINOR_SCALE_INTERVALS: [u8; 7] = [2, 3, 5, 7, 9, 11, 12];
        const MELODIC_MINOR_SCALE_STEPS: [u8; 7] = [2, 1, 2, 2, 2, 2, 1];

        assert_eq!(MELODIC_MINOR_SCALE.pattern(), 0b0000_1101_0101_0110);

        let intervals = MELODIC_MINOR_SCALE.intervals().collect::<Vec<_>>();
        assert_eq!(intervals, MELODIC_MINOR_SCALE_INTERVALS);

        let steps = MELODIC_MINOR_SCALE.steps().collect::<Vec<_>>();
        assert_eq!(steps, MELODIC_MINOR_SCALE_STEPS);

        assert!(!MELODIC_MINOR_SCALE.is_major());
        assert!(MELODIC_MINOR_SCALE.is_minor());

        let d3 = MELODIC_MINOR_SCALE.degree(3);
        assert_eq!(d3, 5);

        let d5 = MELODIC_MINOR_SCALE.degree(5);
        assert_eq!(d5, 9);
    }
}
