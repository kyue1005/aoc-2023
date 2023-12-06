use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut sum: usize = 0;
    
    for line in stdin.lock().lines() {
        let words = line.unwrap();
        let parts: Vec<_> = words.split(": ").collect();
        
        // let mut round = parts[0].split(" ").collect::<Vec<_>>()[1].parse::<usize>().unwrap();
        let sets: Vec<_> = parts[1].split("; ").collect();
            
        let mut red: usize = 0;
        let mut green: usize = 0;
        let mut blue: usize = 0;
        
        for set in sets.iter() {
            let cubes: Vec<_> = set.split(", ").collect();
            
            for cube in cubes.iter() {
                let cube_parts: Vec<_> = cube.split(" ").collect();
                let count = cube_parts[0].parse::<usize>().unwrap();
                let color = cube_parts[1];
                // 12 red cubes, 13 green cubes, and 14 blue cubes
                match color {
                    "red" => {
                        // if count > 12 { round = 0 }
                        if count > red { red = count }
                    },
                    "green" => {
                        // if count > 13 { round = 0 }
                        if count > green { green = count }
                    },
                    "blue" => {
                        // if count > 14 { round = 0 }
                        if count > blue { blue = count }
                    },
                    _ => println!("something else!"),
                }
                // if round == 0 { continue;}
            }
            
            // if round == 0 { continue;}
        }
        println!("{} {} {}", red, green, blue);
        sum += red * green * blue;
        // sum += round;
        println!("{}", sum);
    }
}
