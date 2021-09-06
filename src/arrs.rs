// Arrays- Fixed list where elements are the same data types

//use std::mem;

pub fn run(){
    let mut number: [i32; 5] = [1,2,3,4,5];
    println!("{}", number[1]);
    number[1]  = 19;
    println!("{}", number[1]);

    // to get length
    println!("{}", number.len());

    // get slice
    let slice: &[i32] = &number[0..2];
    println!("{:?}", slice);
}