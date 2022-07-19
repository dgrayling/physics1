use rand::Rng; 
use std::sync::mpsc;

struct Particle {
    x:f64,vx:f64,
    y:f64,vy:f64,
    z:f64,vz:f64,
    mass: f64
}

fn express_mean_square_position(particles: &Vec<Particle>) {
    let mut distances: Vec<f64> = Vec::new();

    for particle in particles.iter() {
        let squared = particle.x * particle.x +
        particle.y * particle.y +
        particle.z * particle.z;

        distances.push(squared.sqrt());
    }

    // println!("Position magnitudes {:?}", distances);

    let sum: f64 = distances.iter().sum();
    let count = distances.len() as f64;

    println!("Position Average {}", sum/count)

}

fn express_mean_square_velocity(particles: &Vec<Particle>) {
    let mut velocity: Vec<f64> = Vec::new();

    for particle in particles.iter() {
        let squared = particle.vx * particle.vx +
        particle.vy * particle.vy +
        particle.vz * particle.vz;

        velocity.push(squared.sqrt());
    }

    // println!("Speeds {:?}", velocity);

    let sum: f64 = velocity.iter().sum();
    let count = velocity.len() as f64;

    println!("Velocity Average {}", sum/count)
}

fn pull_to_center(particles: &mut Vec<Particle>) {
    for particle in particles.iter_mut() {
        particle.vx = -0.1 * particle.x;
        particle.vy = -0.1 * particle.y;
        particle.vz = -0.1 * particle.z;
    }
}

fn time_step(particles: &mut Vec<Particle>) {
    for particle in particles.iter_mut() {
        particle.x += particle.vx;
        particle.y += particle.vy;
        particle.z += particle.vz;
    }
}

fn update_velocities_for_gravity(particles: &mut Vec<Particle>) {
    let length = particles.len();
    for (i, particle) in particles.iter_mut().enumerate() {
        
    }
}

//particles have a mailbox
//all the other particles send their mass and position to the mailbox
//particle calculates its new position and velocity
//particle posts to central controller mailbox, iterate

fn main() {
    let mut rng = rand::thread_rng();
    let mut particles: Vec<Particle> = Vec::new();

    for _ in 0..100 {
        particles.push(Particle{
            x: rng.gen_range(-10.0..10.0), 
            y: rng.gen_range(-10.0..10.0), 
            z: rng.gen_range(-10.0..10.0),
            vx: rng.gen_range(-0.01..0.01), 
            vy: rng.gen_range(-0.01..0.01), 
            vz: rng.gen_range(-0.01..0.01),
            mass: 1.0
        });
    }

    update_velocities_for_gravity(&mut particles);

    express_mean_square_position(&particles);
    express_mean_square_velocity(&particles);

    for _ in 0..40 {
        // express_mean_square_position(&particles);
        // express_mean_square_velocity(&particles);
        pull_to_center(&mut particles);
        time_step(&mut particles);
    }

    express_mean_square_position(&particles);
    express_mean_square_velocity(&particles);

    // need average
}
