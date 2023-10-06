fn main() {
    println!("Hello, world!");
    
    let result: u32= another_function(32);
    println!("the return value is {}",result);
}

fn another_function(x: u32) -> u32{
    println!("this is another function and the input is {}",x);
    x*2
}
