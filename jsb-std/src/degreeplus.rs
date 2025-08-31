use bitflags::bitflags;

bitflags! {
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct DegreePlus: u8 {
        const ROOT = 0;

        const THIRD = 3;
        const THIRD_FLAT = 3 | FLAG_FLAT;

        const FIFTH = 5;
        const FIFTH_FLAT = 5 | FLAG_FLAT;
        const FIFTH_SHARP = 5 | FLAG_SHARP;

        const SEVENTH = 7;
        const SEVENTH_FLAT = 7 | FLAG_FLAT;
        const SEVENTH_DOUBLE_FLAT = 7 | FLAG_DOUBLE_FLAT;
    }
}

const FLAG_SHARP: u8 = 0b0001_0000;
const FLAG_DOUBLE_SHARP: u8 = 0b0010_0000;
const FLAG_FLAT: u8 = 0b0100_0000;
const FLAG_DOUBLE_FLAT: u8 = 0b1000_0000;
const DEGREE_MASK: u8 = 0b0000_1111;

impl DegreePlus {
    pub const fn new_natural(degree: u8) -> Self {
        Self::from_bits_retain(degree)
    }

    pub const fn new_with_sharp(degree: u8) -> Self {
        let d = Self::new_natural(degree).bits();
        Self::from_bits_retain(d | FLAG_SHARP)
    }

    pub const fn new_with_double_sharp(degree: u8) -> Self {
        let d = Self::new_natural(degree).bits();
        Self::from_bits_retain(d | FLAG_DOUBLE_SHARP)
    }

    pub const fn new_with_flat(degree: u8) -> Self {
        let d = Self::new_natural(degree).bits();
        Self::from_bits_retain(d | FLAG_FLAT)
    }

    pub const fn new_with_double_flat(degree: u8) -> Self {
        let d = Self::new_natural(degree).bits();
        Self::from_bits_retain(d | FLAG_DOUBLE_FLAT)
    }

    pub const fn degree(self) -> u8 {
        self.bits() & DEGREE_MASK
    }

    pub const fn is_sharp(self) -> bool {
        self.bits() & FLAG_SHARP != 0
    }

    pub const fn is_double_sharp(self) -> bool {
        self.bits() & FLAG_DOUBLE_SHARP != 0
    }

    pub const fn is_flat(self) -> bool {
        self.bits() & FLAG_FLAT != 0
    }

    pub const fn is_double_flat(self) -> bool {
        self.bits() & FLAG_DOUBLE_FLAT != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree() {
        assert_eq!(DegreePlus::new_natural(0), DegreePlus::ROOT);
    }

    #[test]
    fn test_third() {
        assert_eq!(DegreePlus::THIRD.bits(), 3);

        assert_eq!(DegreePlus::THIRD_FLAT.degree(), 3);
        assert!(DegreePlus::THIRD_FLAT.is_flat());
        assert!(!DegreePlus::THIRD_FLAT.is_sharp());
    }

    #[test]
    fn test_fifth() {
        assert_eq!(DegreePlus::FIFTH.bits(), 5);

        assert_eq!(DegreePlus::FIFTH_FLAT.degree(), 5);
        assert!(DegreePlus::FIFTH_FLAT.is_flat());
        assert!(!DegreePlus::FIFTH_FLAT.is_sharp());

        assert_eq!(DegreePlus::FIFTH_SHARP.degree(), 5);
        assert!(!DegreePlus::FIFTH_SHARP.is_flat());
        assert!(DegreePlus::FIFTH_SHARP.is_sharp());
    }

    #[test]
    fn test_seventh() {
        assert_eq!(DegreePlus::SEVENTH.bits(), 7);

        assert_eq!(DegreePlus::SEVENTH_FLAT.degree(), 7);
        assert!(DegreePlus::SEVENTH_FLAT.is_flat());
        assert!(!DegreePlus::SEVENTH_FLAT.is_sharp());

        assert_eq!(DegreePlus::SEVENTH_DOUBLE_FLAT.degree(), 7);
        assert!(DegreePlus::SEVENTH_DOUBLE_FLAT.is_double_flat());
        assert!(!DegreePlus::SEVENTH_DOUBLE_FLAT.is_flat());
        assert!(!DegreePlus::SEVENTH_DOUBLE_FLAT.is_sharp());
    }
}
