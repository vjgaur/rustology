//Variables hold primitive data or references ot data
//Variables are immutable by default in rust
//Rust is a block-scoped language

pub fn run() {
    let name = "Vijendr";
    let mut age = 37; //making age mutable using keyword mut
    age = 39;
    println!("My Name is {} and I am {}", name, age);
    //Define constant
    const ID: i32 = 001;
    println!("ID:{}", ID);

    //Asssign multiple vars
    let (my_name, my_age) = ("Vijendr", 37);
    println!("{} is {}", my_name, my_age);
}
