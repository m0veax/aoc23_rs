use std::fs;


// TODO: reviews umsetzen

fn main() {

    println!("Loading File");

    // Review: Fehler vll lieber manuell behandeln? expect ist hier ok, aber in großen programmen nicht
    let contents = fs::read_to_string("input1.txt").expect("Couldnt read file");

    //println!("got Content {}", contents);

    let mut sum: u64 = 0;

    // Review: buffered reader ist schneller als split("\n")
    for line in contents.split("\n") {
        println!("Got line Content: {}", line);

        let char_vec: Vec<char> = line.chars().collect();

        //let mod_line = line.replace("one", "1").replace("two", "2").replace("three", "3").replace("four", "4").replace("five", "5").replace("six", "6").replace("seven", "7").replace("eight", "8").replace("nine", "9");

        // TODO need to replace that step by step, first occurence first. So i need to iterate through the string
        // and need to know which indexes that are

        // psoido
        // 
        // e -> no match
        // ei -> no match
        // eig -> no match
        // eigh -> no match
        // eight -> match, replace
        // 8wo -> no match (but two would replace it above which is wrong)

        let mut mod_line: String = String::from("");

        //println!("Modified line Content: {}", mod_line);
        
        let mut result: String = String::from("");

        let mut test_string: String = String::from("");

        // Review: replace all nutzen (hatte ich eingangs nicht gefunden)
        let test_list = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        // müssen wir machen wegen strings wie "twone" <- 2 1
        let replace_list = ["one1one", "two2two", "three3three", "four4four", "five5five", "six6six", "seven7seven", "eight8eight", "nine9nine"];
        // Review: Liste mit Tuples wäre explizit gekoppelt und nicht implizit über den index


        let mut found_replacement: bool = false;

        // Review: replace reicht mit der neuen replace list voll aus
        for(i, &c) in char_vec.iter().enumerate() {

            test_string.push(c.clone());
            //println!("debug string forward: {}", test_string);

            // Review: nach einführung von replace_list nicht mehr notwendig
            for(pos, t) in test_list.iter().enumerate() {
                if test_string.contains(*t) && !found_replacement {
                    let replace : String = String::from(replace_list[pos]);
                    test_string = test_string.replace(t, &replace);
                    //println!("replaced string forward: {}", test_string);
                    found_replacement = true;
                } 
            }

            //println!("modified {}", test_string);
        }

        // Review: test_string kann eingespart werden und direkt auf auf mod_line arbeiten
        mod_line = test_string;

        println!("mod_line before reverse replacing: {}", mod_line);

        // now we need to find the last one and stop after that

        // Review: nach Lösung des "twone" Problems nicht mehr nötig (ärgert mich, dass ich das nicht gesehen hat)
        let mut test_string_rev: String = String::from("");

        let char_ver_modified: Vec<char> = mod_line.chars().collect();

        found_replacement = false;

        for(i, &c) in char_ver_modified.iter().rev().enumerate() {

            test_string_rev.insert(0, c);
            //println!("debug string backwards: {}", test_string_rev);
            

            for(pos, t) in test_list.iter().enumerate() {
                if test_string_rev.contains(*t) && !found_replacement {
                    let replace : String = String::from(replace_list[pos]);
                    test_string_rev = test_string_rev.replace(t, &replace);
                    //println!("replaced string backwards: {}", test_string_rev);
                    found_replacement = true;
                } 
            }

            //println!("modified reverse {}", test_string_rev);
        }

        mod_line = test_string_rev;

        let mod_char_vec: Vec<char> = mod_line.chars().collect();

        println!("search for digits in {}", mod_line);


        // Review: das ist gut. enumerate brauchen wir nicht, da wir pos nicht brauchen
        // first digit
        for (_pos, c) in mod_char_vec.iter().enumerate() {
            if c.is_numeric() {
                println!("found {} going forward", c);
                result.push(*c);
                break;
            }
        }

        // last digit
        for (_pos, c) in mod_char_vec.iter().rev().enumerate() {
            if c.is_numeric() {
                println!("found {} going backward", c);
                result.push(*c);
                break;
            }
        }
        
        if result != "" {
            // Review: unwrap is bad. Besser ist Result oder Option nutzen wenn es ok ist, sonst loggen / error msg
            // Review: unwrap und error ist leise und man bekommt das nicht mit
            let result_int: u64 = result.parse().unwrap();

            println!("result for line is {}", result_int);
    
            sum += result_int;
        }
        
    }

    println!("Sum is {}", sum);
}