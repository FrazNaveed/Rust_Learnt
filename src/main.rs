// mod print;
// mod strcts;


// struct Color{
//    red: u8,
//    green: u8,
//    blue: u8
// }

// tuple struct
struct color(u8,u8,u8);

fn main() {
   // let mut c = Color{
   //    red:255,
   //    green: 0,
   //    blue: 255
   // };
   let c= color(34,32,22);
   println!("{},{},{}", c.0, c.1,c.2);
}
