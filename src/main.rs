use rand::Rng; 
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

struct MassPosition {
    x:f64,
    y:f64,
    z:f64,
    mass: f64
}

struct Particle {
    x:f64,vx:f64,
    y:f64,vy:f64,
    z:f64,vz:f64,
    mass: f64,
    massPositionReceiver: std::sync::mpsc::Receiver<MassPosition>,
}

fn express_mean_square_position(particles: &Vec<Box<Particle>>) {
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

fn express_mean_square_velocity(particles: &Vec<Box<Particle>>) {
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

fn pull_to_center(particles: &mut Vec<Box<Particle>>) {
    for particle in particles.iter_mut() {
        particle.vx = -0.1 * particle.x;
        particle.vy = -0.1 * particle.y;
        particle.vz = -0.1 * particle.z;
    }
}

fn time_step(particles: &mut Vec<Box<Particle>>) {
    for particle in particles.iter_mut() {
        particle.x += particle.vx;
        particle.y += particle.vy;
        particle.z += particle.vz;
    }
}

fn update_velocities_for_gravity(particles: &mut Vec<Box<Particle>>) {
    let length = particles.len();
    for (i, particle) in particles.iter_mut().enumerate() {
        
    }
}

fn get_particle_and_mass_position_sender(rng : &mut rand::prelude::ThreadRng) -> (Sender<MassPosition>,std::boxed::Box<Particle>) {
    let (tx, rx) = mpsc::channel::<MassPosition>();

    let thing = Particle {
        x: rng.gen_range(-10.0..10.0), 
        y: rng.gen_range(-10.0..10.0), 
        z: rng.gen_range(-10.0..10.0),
        vx: rng.gen_range(-0.01..0.01), 
        vy: rng.gen_range(-0.01..0.01), 
        vz: rng.gen_range(-0.01..0.01),
        mass: 1.0,
        massPositionReceiver: rx,
    };

    let boxedThing = Box::new(thing);

    return (tx,boxedThing);
}

//particles have a mailbox
//all the other particles send their mass and position to the mailbox
//particle calculates its new position and velocity
//particle posts to central controller mailbox, iterate

fn main() {
    let mut rng = &mut rand::thread_rng();
    let mut particles: Vec<Box<Particle>> = Vec::new();

    let mut massPositionSenderVector: Vec<std::sync::mpsc::Sender<MassPosition>> = Vec::new();
    


    for _ in 0..100 {
        let (sender, particle) = get_particle_and_mass_position_sender(rng);
        massPositionSenderVector.push(sender);
        particles.push(particle);
    }

    // update_velocities_for_gravity(&mut particles);

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
