use std::io;
use std::time::{Instant};

fn main() {
    println!("Enter something:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("this is bad");

    let something = input.trim(); 
    let mut copy: String = ' '.to_string();
    let mut int_char: u8 = 19; //start around ascii (space)
    let mut our_char: char = int_char as char;
    let start = Instant::now(); //timing our project should be linear-ish O(n)

    for i in 0..something.len() {
        while our_char != something.chars().nth(i).unwrap(){
            copy.push(our_char);
            println!("{}",copy);
            copy.pop();
            int_char += 1;
            our_char = int_char as char;
        }
        int_char = 19;
        copy.push(our_char);
        println!("{}",copy);
    }

    for _ in 0..3 {
        println!();
    }
    let duration = start.elapsed();
    println!("Your message has been ASCII brute forced in {:?}", duration);
}
