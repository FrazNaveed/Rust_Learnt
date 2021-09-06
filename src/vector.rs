// vectors- resizeable arrays

pub fn run(){
    let mut number: Vec<i32> = vec![1,2,3,4,5];
    println!("{}", number[1]);
    number[1]  = 19;
    println!("{}", number[1]);

    // to get length
    println!("{}", number.len());

    // get slice
    let slice: &[i32] = &number[0..2];
    println!("{:?}", slice);

    // add on to vector
    number.push(6);
    number.push(7);
    println!("{:?}", number);

    // loop through array
    for x in number.iter(){
        println!(
            "Number is: {}",x);
    }
    // loop and mutate values
    for x in number.iter_mut(){
        *x *=2;
        // println!("Number is {}", number);
    }
}