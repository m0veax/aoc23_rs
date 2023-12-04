use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Stones {
    red: u8,
    green: u8,
    blue: u8
}



impl Stones {

    fn max_stones(&mut self, color: &str, amount: u8) {
        

        match color {
            "red" => {
                if amount > self.red {
                    self.red = amount
                }
            },
            "green" => {
                if amount > self.green {
                    self.green = amount
                }
            },
            "blue" => {
                if amount > self.blue {
                    self.blue = amount
                }
            },
            &_ => todo!(),
        }
    }
}


fn main() {

    let mut vec_stones: Vec<(u8, Stones)> = vec![];
    let file: String = String::from("input1.txt");

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                process_line(&ip, &mut vec_stones);
            }
        }
    }

    check_possible_games(&vec_stones);
}

fn check_possible_games(vec_stones: &Vec<(u8, Stones)>) {

    let mut id_sum: u64 = 0;

    for game in vec_stones {
        let (id, stones) = game;
        
        println!("checking game {id}");
        dbg!(stones);
        println!("{}", stones.red);

        if stones.red <= 12 && stones.green <= 13 && stones.blue <= 14 {
            id_sum += *id as u64;
        }
    }

    println!("idsum: {id_sum}");
}

fn process_line(line: &String, vec_stones: &mut Vec<(u8, Stones)>) {
    let game_parts: Vec<&str> = line.split(": ").collect();
    let id_parts: Vec<&str> = game_parts[0].split(" ").collect();
    let id: u8 = id_parts[1].parse().unwrap();
    let game_content: Vec<&str> = game_parts[1].split("; ").collect();

    let mut game_stones = Stones {
        red: 0,
        green: 0,
        blue: 0
    };

    for game in game_content {
        println!("specific game content: {game}");
        let stone_count_parts: Vec<&str> = game.split(", ").collect();
        dbg!(&stone_count_parts);
        for stone_count in &stone_count_parts {
            println!("stone count string {stone_count}");
            let stone_count_collection: Vec<&str> = stone_count.split(" ").collect();
            let color = String::from(stone_count_collection[1]);
            let amount: u8 = stone_count_collection[0].parse().unwrap();
            println!("Maxing {amount} {color} stones");
            game_stones.max_stones(&color, amount);
        }
    
    }

    dbg!(&game_stones);

    vec_stones.push((id, game_stones));
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}