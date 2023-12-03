use std::fs;




fn main() {

    println!("Loading File");

    let contents = fs::read_to_string("input1.txt").expect("Couldnt read file");

    println!("got Content {}", contents);

    let mut sum: u64 = 0;

    for line in contents.split("\n") {
        println!("Got line Content: {}", line);
        let char_vec: Vec<char> = line.chars().collect();

        let mut result: String = String::from("");

        // first digit
        for (_pos, c) in char_vec.iter().enumerate() {
            if c.is_numeric() {
                println!("found {} going forward", c);
                result.push(*c);
                break;
            }
        }

        // last digit
        for (_pos, c) in char_vec.iter().rev().enumerate() {
            if c.is_numeric() {
                println!("found {} going backward", c);
                result.push(*c);
                break;
            }
        }
        
        if result != "" {
            let result_int: u64 = result.parse().unwrap();

            println!("result for line is {}", result_int);
    
            sum += result_int;
        }
        
    }

    println!("Sum is {}", sum);
}