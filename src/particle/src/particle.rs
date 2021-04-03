
pub type ParticleID = u8;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Particle {
    // The ID of the particle
    pub id: ParticleID,

    // Physical properties
    pub mass: u32,
    pub color: [u8; 4],
    pub mu: u32,
    pub boiling_point: u32,
    pub melting_point: u32,
}

impl Particle {

    fn new(id: ParticleID, mass: u32, color: &[u8; 4], mu: u32, boiling_point: u32, melting_point: u32) -> Self {
        Particle {
            id,
            mass,
            color: color.clone(),
            mu,
            boiling_point,
            melting_point,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::particle::Particle;
    
    #[test]
    fn it_works() {
        let id = 2;
        let mass = 0;
        let color = [1, 2, 3, 4];
        let mu = 12;
        let boiling_point = 0;
        let melting_point = 0;
        let p = Particle::new(id, mass, &color, mu, boiling_point, melting_point);
        // println!("{:?}", p);
        dbg!(p);
        println!("hello world {}", 12);
        assert_eq!(p.id, id);
        assert_eq!(p.mass, mass);
        assert_eq!(p.color, color);
        assert_eq!(p.mu, mu);
        assert_eq!(p.boiling_point, boiling_point);
        assert_eq!(p.melting_point, melting_point);
    }
}
