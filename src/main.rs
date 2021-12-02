use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*;

fn main() { //-> std::io::Result<()> {

    if let Ok(lines) = read_lines("src/names.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(names) = line {
                println!("{}", names);



    //let mut index = 0;
    //let mut file = File::open("src/names.txt")?;
    //let mut contents = String::new();
    //file.read_to_string(&mut contents)?;
    //println!("{}", contents);
}
}
}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
