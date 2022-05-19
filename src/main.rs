use rand::Rng;

struct Particle {
    x:f64,vx:f64,
    y:f64,vy:f64,
    z:f64,vz:f64
}

fn express_mean_square_position(particles: &Vec<Particle>) {
    let mut distances: Vec<f64> = Vec::new();

    for particle in particles.iter() {
        let squared = particle.x * particle.x +
        particle.y * particle.y +
        particle.z * particle.z;

        distances.push(squared.sqrt());
    }

    println!("Position magnitudes {:?}", distances);
}

fn express_mean_square_velocity(particles: &Vec<Particle>) {
    let mut velocity: Vec<f64> = Vec::new();

    for particle in particles.iter() {
        let squared = particle.vx * particle.vx +
        particle.vy * particle.vy +
        particle.vz * particle.vz;

        velocity.push(squared.sqrt());
    }

    println!("Speeds {:?}", velocity);
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut particles: Vec<Particle> = Vec::new();
    let mut i = 0;

    while i < 100 {
        particles.push(Particle{
            x: rng.gen_range(-10.0..10.0), 
            y: rng.gen_range(-10.0..10.0), 
            z: rng.gen_range(-10.0..10.0),
            vx: rng.gen_range(-0.01..0.01), 
            vy: rng.gen_range(-0.01..0.01), 
            vz: rng.gen_range(-0.01..0.01),
        });
        i = i + 1;
    }

    let mut distances: Vec<f64> = Vec::new();

    for particle in particles.iter() {
        let squared = particle.x * particle.x +
        particle.y * particle.y +
        particle.z * particle.z;

        distances.push(squared.sqrt());
    }

    express_mean_square_position(&particles);
    express_mean_square_velocity(&particles);
}
