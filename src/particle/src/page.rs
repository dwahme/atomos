use std::mem::size_of;

const ID_MASK: u8 = 0b00111111;

/// A page holding particle information
/// Occupies exactly 4KB in memory
#[derive(Debug)]
pub struct Page {
    /// The ID of the particle
    pub map: [u8; 4096],
}

impl Page {
    /// Create a new page
    pub fn new(map: &[u8; 4096]) -> Self {
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
}


/// A Page storing particle data
#[derive(Debug)]
pub(crate) struct ParticlePage {
    /// The page holding the list of particles
    pub page: Page,
}

impl ParticlePage {
    /// Create a new page
    pub fn new(map: &[u8; 4096]) -> Self {
        ParticlePage { page: Page::new(map) }
    }

    /// Set a particle at index 
    pub fn set_particle(&mut self, index: usize, particle: u8) {
        self.page.map[index] = particle;
    }

    /// Get a particle ID at index 
    pub fn get_particle_id(&self, idx: usize) -> u8 {
        self.page.map[idx] & ID_MASK
    }

    /// Get a particle ID at index 
    pub fn set_particle_id(&mut self, idx: usize, id: u8) {
        let old = self.page.map[idx] & ID_MASK;

        // Keep the metadata
        let new = (old & !ID_MASK) & (id & ID_MASK);
        
        self.page.map[idx] = new
    }
}

#[derive(Debug)]
pub(crate) struct PhysicsPage {
    pub pages: [Page; 2]
}

impl PhysicsPage {
    /// Creates a new physics page 
    pub fn new(maps: [Page; 2]) -> Self {
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
}
