use std::collections::HashMap;
use crate::common::ParticleID;
use crate::particle::Particle;

fn init_types() -> HashMap<ParticleID, Particle> {
    let mut types: HashMap<ParticleID, Particle> = HashMap::new();

    // void
    types.insert(0, Particle::new(0, 0, &[0, 0, 0, 255], 0, 0));

    // air
    types.insert(1, Particle::new(1, 29, &[222, 235, 255, 100], 79, 58)); 
                                  // melting not really applicable

    // water
    types.insert(2, Particle::new(2, 18, &[65, 120, 205, 150], 373, 273));

    // sand
    types.insert(3, Particle::new(3, 60, &[247, 228, 183, 255], 2503, 1823));

    // iron (cast)
    types.insert(4, Particle::new(4, 56, &[130, 130, 130, 255], 2773, 1473,));

    types
}