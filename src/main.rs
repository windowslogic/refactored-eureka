use std::io;
use std::fs;

fn main() {
    
    let array = [1,2,3,4,5];
    
    println!("Please ender an array index");
    
    let mut index = fs::read("src/names.txt");
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    
    let element = array[index];
    
    println!("The value of the element at index {} is: {}", index, element);
    
}
