use particle::{particle::Particle, chunks::Chunks};
use render::render::render;
use particle::common::{NUM_PHYSICS_PAGE, MAX_PARTICLES, MAX_X, MAX_Y, MAX_Z, ParticleID};

fn main() {
    let chunks = Chunks::new();
    for x in 0..MAX_X {
        for y in 0..MAX_Y {
            for z in 0..MAX_Z {
                let particle_page = chunks.get_chunk_particle(x, y, z);
                let mut particle_page_mut = particle_page.write().unwrap();
                particle_page_mut.set_particle(2, 1);

                let physics_pages = chunks.get_chunk_physics(x, y, z);
                let mut physics_pages = physics_pages.write().unwrap();
                

            }
        }
    }

    println!("Hello, world!");
    render();
}
