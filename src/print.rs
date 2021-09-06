pub fn run() {
    // basic formatting
    println!("{} my name is {}", "hello", "fraz");

    //positional arguments
    println!("{0} is being {0} He is {1}", "brad", "good");

    // named arguments
    println!("{name1} and {name2}", name1="fraz", name2="john");

    // placeholder traits
    println!("Binary: {:b} Hexa: {:x} Octal: {:o}", 10,10, 100);

    // placeholder debug
    println!("{:?}" , (10 ,true));

    // basic maths
    println!("10 + 10 is {}", 10+10);
}
