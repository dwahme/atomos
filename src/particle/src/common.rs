
pub type ChunkID = u32;
pub type ParticleID = u8;
pub type ObjectID = u8;

pub const ID_MASK: u8 = 0b00111111;
pub const NUM_PHYSICS_PAGE: usize = 4;
pub const MAX_PARTICLES: usize = 4096;
