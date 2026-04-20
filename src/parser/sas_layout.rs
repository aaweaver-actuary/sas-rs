pub enum SasLayout {
    Bit32,
    Bit64,
}

impl SasLayout {
    pub fn to_bits(&self) -> u8 {
        match self {
            SasLayout::Bit32 => 32,
            SasLayout::Bit64 => 64,
        }
    }

    pub fn from_bits(bits: u8) -> Option<Self> {
        match bits {
            32 => Some(SasLayout::Bit32),
            64 => Some(SasLayout::Bit64),
            _ => None,
        }
    }

    pub fn as_code(&self) -> u8 {
        match self {
            SasLayout::Bit32 => 0x22,
            SasLayout::Bit64 => 0x33,
        }
    }

    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0x22 => Some(SasLayout::Bit32),
            0x33 => Some(SasLayout::Bit64),
            _ => None,
        }
    }

    pub fn is_bit32(&self) -> bool {
        matches!(self, SasLayout::Bit32)
    }

    pub fn is_bit64(&self) -> bool {
        matches!(self, SasLayout::Bit64)
    }

    pub fn is_valid(&self) -> bool {
        self.is_bit32() || self.is_bit64()
    }

    pub fn n_bytes(&self) -> usize {
        match self {
            SasLayout::Bit32 => 4,
            SasLayout::Bit64 => 8,
        }
    }

    pub fn word_size(&self) -> u8 {
        match self {
            SasLayout::Bit32 => 32,
            SasLayout::Bit64 => 64,
        }
    }

}
