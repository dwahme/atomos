use std::collections::HashMap;
use particle::common::{ChunkID, ObjectID, ParticleID};
use particle::page::Page;

/// An object made up on many particles
/// Can span multiple chunks
/// Has physics
pub(crate) struct Object {
    // The list of chunks associated with this object
    // as well as the object's ID in that chunk
    pub chunk_ids: HashMap<ChunkID, ObjectID>,

    // Physical properties
    pub total_mass: u32,
    pub velocities: [i32; 3],
    pub moment_of_inertia: u32,
    pub angular_velocities: [i32; 3],
}

impl Object {

    // pub fn new() -> Self {
    //     Object { chunk_ids: HashMap::new(), total_mass: 0, velocities: [0.0; 3] }
    // }

    /// Registers a particle with an object
    /// Creates a new ObjectID for the chunk if necessary
    pub fn add_particle(&mut self, chunk_id: ChunkID, p_id: ParticleID) {
        // TODO- register the particle with the object
        match self.chunk_ids.get(&chunk_id) {
            Some(c) => (),
            None => ()
        }

        // TODO- add particle mass to the total mass of the object
        // TODO- add particle velocity to the object?

        // TODO- update moment of inertia
    }

    /// Deregisters a particle with an object
    /// Removed the ObjectID for the chunk if necessary
    pub fn remove_particle(&mut self, chunk_id: ChunkID, p_id: ParticleID) {
        // TODO- deregister the particle with the object
        match self.chunk_ids.get(&chunk_id) {
            Some(c) => (),
            None => ()
        }

        // TODO- remove particle mass to the total mass of the object
        // TODO- remove particle velocity to the object?

        // TODO- update moment of inertia
    }

    /// Accelerates an object given forces in XYZ directions
    pub fn accelerate(&mut self, forces: &[i32; 3]) {
        for i in 0..3 {
            self.velocities[i] += forces[i] / (self.total_mass as i32)
        }
    }

    // /// Moves an object
    // pub fn r#move(&mut self, chunks: &[Page]) {
    //     for (c_id, o_id) in self.chunk_ids.into_iter() {
    //         // chunk_move(chunks, c_id, o_id, &self.velocities)
    //     }
    // }
}
