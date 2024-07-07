use std::fs::read_to_string;

use rand::Rng;

fn lines(path: &str) -> std::io::Result<Vec<String>>{
    let mut adjectives = Vec::with_capacity(8980);
    
    for line in read_to_string(path)?.lines() {
        adjectives.push(String::from(line));
    }

    Ok(adjectives)
}


fn main() -> std::io::Result<()> {
    let (adjectives, animals) = (
        lines("adjectives.txt")?,
        lines("animals.txt")?
    );

    let mut rng = rand::thread_rng();
    let (i, j) = (
        rng.gen_range(0..adjectives.len()),
        rng.gen_range(0..animals.len())
    );

    let (adj, animal) = (&adjectives[i], &animals[j]);

    println!("{}_{}", adj, animal);

    Ok(())
}
