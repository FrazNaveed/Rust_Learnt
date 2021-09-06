// variables hold primitive data or reference to data
// variables are immutable by default
// Rust is a block scoped language. 

pub fn run(){

    let name = "fraz";
    // mutable variable
    let mut age = 24;
    age = 25;
    println!("my name is {} and my age is {}", name, age);

    // Assign multiple vars
    let (myname, myage) = ("fraz", 24+4);
    println!("my name is {} and my age is {}", myname, myage);
   
}