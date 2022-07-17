#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum AreaSize {
    Small,
    Medium,
    Large,
    Custom(SizeU),
}

impl Default for AreaSize {
    fn default() -> Self {
        AreaSize::Small
    }
}

impl AreaSize {
    pub fn to_size(self) -> SizeU {
        match self {
            AreaSize::Small => SizeU::SMALL,
            AreaSize::Medium => SizeU::MEDIUM,
            AreaSize::Large => SizeU::LARGE,
            AreaSize::Custom(c) => c,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct SizeU {
    pub x: u32,
    pub y: u32,
}

impl SizeU {
    pub const SMALL: SizeU = SizeU { x: 256, y: 256 };
    pub const MEDIUM: SizeU = SizeU { x: 384, y: 384 };
    pub const LARGE: SizeU = SizeU { x: 512, y: 512 };
}

#[cfg(feature = "game")]
impl From<SizeU> for bevy::math::UVec2 {
    fn from(su: SizeU) -> Self {
        bevy::math::UVec2::new(su.x, su.y)
    }
}
