/* 
Primitive Types:
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples // list 
Arrays
*/ 

pub fn run(){
    let  y = 123;
    let z = true;
    let n = 12.3;

    // to tell explicitly
    let c: i32 = 12345;
    let b: bool = false;

    let ch = 'a';
    let face = '\u{1F600}'; // unicode

    println!("{:?}", (ch, b, c, face));

}