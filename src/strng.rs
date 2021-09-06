// Primitive str= Immutable fixed-length string somewhere in memory
// String= Growable, heap-allocated data structure - use when need to modify 

pub fn run(){

    // let name = ("fraz"); then we wont be able to update. 

    let mut name  = String::from("fraz ");
    println!("{}", name);
   // println!("{}", name.len());

    // if you want to push 
    name.push('n');
    println!("{}", name);

    // if you want to push string
    name.push_str("aveed");
    println!("{}", name);

    // contains
    println!("{}", name.contains("raz"));

}