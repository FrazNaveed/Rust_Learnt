pub fn run(){

    //bind function to variables
    let val = add(5,5);
    greeting("hello", "fraz", val);

    // closures in rust
    let n3:i32 = 16;
    let add_num= |n1:i32, n2:i32| n1+n2+n3;
    println!("Closure Sum: {}", add_num(3,3));
}
fn greeting(greet: &str, name: &str, val: i32){
    println!("{},{} good morning. The value is {}", greet, name, val);
}
fn add(num1: i32, num2: i32) -> i32{
    num1+num2
}