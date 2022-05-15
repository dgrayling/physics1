struct Particle {
    x:i64,
    y:i64,
    z:i64
}

fn main() {
    let mut vector: Vec<Particle> = Vec::new();

    let mut i = 0;
    while i < 100 {
        vector.push(Particle{x: 0, y: 0, z: 0});
        i = i + 1;
    }

    println!("Hello, world!");
}
