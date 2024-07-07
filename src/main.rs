use std::{env, fs::read_to_string};
use rand::Rng;

fn lines(path: &str) -> std::io::Result<Vec<String>>{
    let mut vals: Vec<String> = Vec::with_capacity(8980);
    
    for line in read_to_string(path)?.lines() {
        vals.push(String::from(line));
    }

    Ok(vals)
}


fn main() -> std::io::Result<()> {
    let (adjectives, animals) = (
        lines("adjectives.txt")?,
        lines("animals.txt")?
    );

    let args: Vec<String> = env::args().collect();
    
    let count =
        match args.get(1) {
            Some(arg) =>
                match arg.parse::<i32>() {
                    Ok(count) if count <= 999 => count,
                    _ => 1,
                },
            None => 1
        };


    let mut rng = rand::thread_rng();

    for _ in 0..count {
        let (i, j) = (
            rng.gen_range(0..adjectives.len()),
            rng.gen_range(0..animals.len())
        );
    
        let (adj, animal) = (&adjectives[i], &animals[j]);
    
        println!("{}_{}", adj, animal);
    }

    Ok(())
}
