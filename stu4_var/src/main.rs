/**
 * @file main.rs
 * @author lihuax
 * @version 0.1
 * @descritpion
 */

fn main(){
    let x = 5;
    println!("the value of x is: {x}");
    //x = 6;
    //println!("the value of x is: {x}");
    //shadowing
    let x = 15;
    println!("the value of x is: {x}");
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 7;
    println!("the value of y is: {y}");
    let space = "    ";
    //let space = space.len();
    println!("the value of space is: '{space}'");

    /*
     *  int: -(2^(n-1)) ~ 2^(n-1)-1
     *  unsigned int: 0 ~ 2^n-1
     *  length(bit)     int     unsignedInt
     *  8               i16     u16
     *  32              i32     u32
     *  64              i64     u63
     *  128             i128    u128
     *  arch            isize   usize
     *
     *  dec: 100_000
     *  Hex: 0xff
     *  Oct: 0o77
     *  Bin: 0b1100_11--
     *  Byte: b'A'
     */
    let num: u32 = 3;
    let float: f32 = 3.12;
    println!("the numbres are {num}, {float}");
    let c: char = 'Z';
    let fox: char = '😻';
    println!("chars: {c}, and {fox}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("the number one is {}", tup.0);

    let a = [9,8,7,6,5];
    println!("Please input an index:");
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");
    println!("The value of the element at index is {}", a[index]);

}
