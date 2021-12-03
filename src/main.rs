use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {

    let mut index = 0;
    let args: Vec<String> = env::args().collect();
    let arg = &args[1];
    let count: i32 = match arg.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
                    eprintln!("error: argument not an integer");
                    return;
        }
        };
        
        println!("{}", count);
    
    while index < count {
    if let Ok(lines) = read_lines("src/names.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(names) = line {
                println!("{}", names);
                
                index += 1;
}
}
}
}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
