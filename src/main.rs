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
    let (adjectives, animals, flowers, colors) = (
        lines("adjectives.txt")?,
        lines("animals.txt")?,
        lines("flowers.txt")?,
        lines("colors.txt")?
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
        let (i, j, k, l) = (
            rng.gen_range(0..adjectives.len()),
            rng.gen_range(0..animals.len()),
            rng.gen_range(0..colors.len()),
            rng.gen_range(0..flowers.len())
            
        );
    
        let (adj, animal, color, flower) = (
            &adjectives[i],
            &animals[j],
            &colors[k],
            &flowers[l]
        );
        
        println!("{}_{}", adj.to_ascii_lowercase(), animal.to_ascii_lowercase());
        println!("{}_{}", color.to_ascii_lowercase(), flower.to_ascii_lowercase());
    }

    Ok(())
}
