// Tuples group together values of different types
// Max 12 elements

pub fn run(){
    let person: (&str, &str, i8) = ("fraz", "nv", 14);
    println!("{}{} is aged {}", person.0, person.1, person.2); 
}