use std::io::{self, Read};

fn main(){
    println!("calculator");
    calculate();
}

fn calculate(){
    let mut num1 = String::new();
    let mut num2 = String::new();

    io::stdin().read_line(&mut num1)
        .expect("Failed to read number");
    let number1: i64 = num1.trim().parse()
        .expect("Please enter a valid number");



    println!("num1: {}",num1);
    
    let mut num2 = String::new();

    io::stdin().read_line(&mut num2)
        .expect("Failed to read number");
    let number2: i64 = num2.trim().parse()
        .expect("Please enter a valid number");
  

    
    println!("num2: {}", num2);
    let sum  = number1 + number2;
    println!("Output: {}" ,sum);

    
}