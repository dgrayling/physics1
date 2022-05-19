use rand::Rng;

struct Particle {
    x:f64,vx:f64,
    y:f64,vy:f64,
    z:f64,vz:f64
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut particles: Vec<Particle> = Vec::new();
    let mut i = 0;

    let x = rng.gen_range(-10.0..10.0);

    while i < 100 {
        particles.push(Particle{
            x: rng.gen_range(-10.0..10.0), 
            y: rng.gen_range(-10.0..10.0), 
            z: rng.gen_range(-10.0..10.0),
            vx: rng.gen_range(-10.0..10.0), 
            vy: rng.gen_range(-10.0..10.0), 
            vz: rng.gen_range(-10.0..10.0),
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

    println!("Particle 1 {}", distances[0]);
}
