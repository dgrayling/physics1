struct particle {
    x:i64,
    y:i64,
    z:i64
}

fn main() {
    let mut vector: Vec<particle> = Vec::new();
    vector.push(particle{x: 0, y: 0, z: 0});
    vector.push(particle{x: 0, y: 0, z: 1});

    println!("Hello, world!");
}
