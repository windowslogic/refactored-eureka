use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let mut index = 0;
    let mut file = File::open("src/names.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
