use std::collections::HashMap;
use crate::common::{ChunkID, ObjectID};

/// An object made up on many particles
/// Can span multiple chunks
/// Has physics
pub(crate) struct Object {
    // The ID of the particle
    pub id: u32,

    // The list of chunks associated with this object
    // as well as the object's ID in that chunk
    pub chunk_ids: HashMap<ChunkID, ObjectID>,

    // Physical properties
    pub total_mass: u32,
}
