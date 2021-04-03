use crate::common::NUM_PHYSICS_PAGE;
use crate::page::{Page, ParticlePage, PhysicsPage};
use crate::particle::Particle;
use std::sync::{Arc, RwLock};

const MAX_X: usize = 2;
const MAX_Y: usize = 2;
const MAX_Z: usize = 2;

/// The big storage for all the particles that are being tracked
pub struct Chunks {
    /// All the pages that make up the chunks
    particle_pages: Vec<Arc<RwLock<ParticlePage>>>,
    physics_pages: Vec<Arc<RwLock<PhysicsPage>>>,
}

impl Chunks {

    pub fn new() -> Self {
        let mut particle_pages = Vec::new();
        let mut physics_pages = Vec::new();

        for _ in 0..(MAX_X * MAX_Y * MAX_Z) {
            particle_pages.push(Arc::new(RwLock::new(ParticlePage::new(&[0; 4096]))));
            physics_pages.push(Arc::new(RwLock::new(PhysicsPage::new([Page::new(&[0; 4096]); NUM_PHYSICS_PAGE]))));
        }

        Chunks { particle_pages, physics_pages }
    }

    /// Gets a particle info page at location x/y/z
    pub fn get_chunk_particle(&self, x: usize, y: usize, z: usize) -> Arc<RwLock<ParticlePage>> {
        let idx = z * MAX_Y * MAX_X + y * MAX_X + x;
        self.particle_pages[idx].clone()
    }

    /// Gets a physics info page at location x/y/z
    pub fn get_chunk_physics(&self, x: usize, y: usize, z: usize) -> Arc<RwLock<PhysicsPage>> {
        let idx = z * MAX_Y * MAX_X + y * MAX_X + x;
        self.physics_pages[idx].clone()
    }
}