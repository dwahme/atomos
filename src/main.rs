use particle::{particle::Particle, chunks::Chunks};
use render::render::render;

fn main() {
    let chunks = Chunks::new();
    println!("Hello, world!");
    render();
}
