//fn main() {
//    let x = 5;
//    println!("The value of x is: {x}");
//    x = 6;
//    println!("The value of x is: {x}");
//}


fn main(){
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let float: f32 = 1.0;
    println!("{float}");
    let double: f64 = 1.0;
    println!("double = {double}");
    let int: i32 = 1;
    println!("int = {int}");
    let long: i64 = 1;
    println!("long = {long}");
    let chr: char = 'H';
    println!("chr = {chr}");
    let flag: bool = true;
    println!("flag = {flag}");
    let tuple: (i32, f64, u8) = (3, 5.2, 1);
    let (x, y, z) = tuple;
    println!("x = {x}, y = {y}, z = {z}");
    let three = tuple.0;
    println!("three = {three}");
    let five_point_two = tuple.1;
    println!("five_point_tow = {five_point_two}");
    let one = tuple.2;
    println!("one = {one}");
    let array = [1, 2, 3, 4, 5];// equal let array: [i32; 5] = [1, 2, 3, 4, 5];
    let array = [5; 5]; // equal let array = [5, 5, 5, 5, 5];
}
