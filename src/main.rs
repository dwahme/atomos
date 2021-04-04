use particle::{particle::Particle, chunks::Chunks};
use render::render::render;

fn main() {
    let chunks = Chunks::new();
    for page in chunks.particle_pages {
        let mut particle_page = page.write().unwrap();
    }

    println!("Hello, world!");
    render();
}
