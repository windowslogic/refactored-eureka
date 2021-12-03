use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut index = 1;
    
    if let Ok(lines) = read_lines("src/names.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(names) = line {
                println!("{}. {}", index, names);
                
                index += 1;
}
}
}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
