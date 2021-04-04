use crate::common::{ID_MASK, MASS_MASK, NUM_PHYSICS_PAGE, MAX_PARTICLES, ParticleID, MAX_PARTICLES_EDGE};
use physics::Minifloat;

/// A page holding particle information
/// Occupies exactly 4KB in memory
#[derive(Debug, Copy, Clone)]
pub struct Page {
    /// The ID of the particle
    pub map: [u8; MAX_PARTICLES],
}

impl Page {
    /// Create a new page
    pub fn new(map: &[u8; MAX_PARTICLES]) -> Self {
        Page { map: map.clone() }
    }

    /// Get a value at index
    pub fn get_value(&self, idx: usize) -> u8 {
        self.map[idx]
    }

    /// Get a particle at index
    pub fn set_value(&mut self, idx: usize, value: u8) {
        self.map[idx] = value
    }

    pub fn get_idx_from_xyz(x: usize, y: usize, z: usize) -> usize {
        z * MAX_PARTICLES_EDGE * MAX_PARTICLES_EDGE + y * MAX_PARTICLES_EDGE + x
    }

    pub fn get_xyz_from_idx(idx:usize) -> (usize, usize, usize) {
        let max_particles_square = MAX_PARTICLES_EDGE & MAX_PARTICLES_EDGE;
        let z = idx / max_particles_square as usize;
        let tmp = (idx % max_particles_square);
        let y = tmp / MAX_PARTICLES_EDGE as usize;
        let x = tmp % MAX_PARTICLES_EDGE as usize;
        (x, y, z)
    }
}


/// A Page storing particle data
#[derive(Debug)]
pub struct ParticlePage {
    /// The page holding the list of particles
    pub page: Page,
}

impl ParticlePage {
    /// Create a new page
    pub fn new(map: &[u8; MAX_PARTICLES]) -> Self {
        ParticlePage { page: Page::new(map) }
    }

    /// Set a particle at index
    pub fn set_particle(&mut self, index: usize, particle: ParticleID) {
        self.page.map[index] = particle;
    }

    /// Get a particle ID at index
    pub fn get_particle_id(&self, idx: usize) -> u8 {
        self.page.map[idx] & ID_MASK
    }

    /// Get a particle ID at index
    pub fn set_particle_id(&mut self, idx: usize, id: ParticleID) {
        let old = self.page.map[idx] & ID_MASK;

        // Keep the metadata
        let new = (old & !ID_MASK) & (id & ID_MASK);

        self.page.map[idx] = new
    }
}

#[derive(Debug)]
pub struct PhysicsPage {
    pub pages: [Page; NUM_PHYSICS_PAGE]
}

impl PhysicsPage {
    /// Creates a new physics page
    pub fn new(maps: [Page; NUM_PHYSICS_PAGE]) -> Self {
        PhysicsPage { pages: maps }
    }

    /// Get a data at page and index
    pub fn get_particle_data(&self, page_num: usize, idx: usize) -> u8 {
        self.pages[page_num].get_value(idx)
    }

    /// Set physics data at page and index
    pub fn set_particle_data(&mut self, page_num: usize, idx: usize, data: u8) {
        self.pages[page_num].set_value(idx, data)
    }

    /// Returns the mass of a particle given an index
    pub fn get_mass(&self, x: usize, y: usize, z: usize) -> u8 {
        let idx = Page::get_idx_from_xyz(x, y, z);
        self.pages[0][idx] & MASS_MASK
    }

    // There's prob macros for this but whatever

    /// Returns a float velocity X of a particle given index
    pub fn get_vel_x(&self, idx: usize) -> f32 {
        let data = self.pages[1][idx] as i8;
        Minifloat::new_from_i8(data).to_float()
    }

    /// Returns a float velocity Y of a particle given index
    pub fn get_vel_y(&self, idx: usize) -> f32 {
        let data = self.pages[2][idx] as i8;
        Minifloat::new_from_i8(data).to_float()
    }

    /// Returns a float velocity Z of a particle given index
    pub fn get_vel_z(&self, idx: usize) -> f32 {
        let data = self.pages[3][idx] as i8;
        Minifloat::new_from_i8(data).to_float()
    }

    /// Sets the velocity x of a particle given the index and velocity
    pub fn set_vel_x(&mut self, idx: usize, vel: f32) {
        self.pages[1][idx] = Minifloat::new(vel).to_u8();
    }

    /// Sets the velocity y of a particle given the index and velocity
    pub fn set_vel_y(&mut self, idx: usize, vel: f32) {
        self.pages[2][idx] = Minifloat::new(vel).to_u8();
    }

    /// Sets the velocity z of a particle given the index and velocity
    pub fn set_vel_z(&mut self, idx: usize, vel: f32) {
        self.pages[3][idx] = Minifloat::new(vel).to_u8();
    }

}
