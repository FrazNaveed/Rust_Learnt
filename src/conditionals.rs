pub fn run(){
    let age = 16;
    // if else
    if age>18{
        println!("What would you like?");
    }
    else{
        println!("grow up first");
    }

    // shorthand IF
    let is_of_age = if age >=21 {true}else{false};
    println!("{}", is_of_age);
}