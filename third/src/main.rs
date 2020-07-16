use std::io::{stdin,stdout,write};
fn ready(input:& String){
    stdout().flush().expect("sorry");
    stdin().read_line(input).expect("your error occured");
    
}
fn main (){
    println!("welcome to khan calculator");
    
    let num1 = String::new();
    let num2 = String::new();
    let operator = String::new();
    
    println!("type first number ?");
    ready(&mut num1);
    println!("type your operator");
    ready(&mut operator);
    println!("type your second number ?");
    ready(&mut num2)
    
}
