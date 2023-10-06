/**
 * @file main.rs
 * @author lihuax
 * @version 0.1
 * @descritpion
 */
fn main() {
    let mut s1 = String::from("hello");

    //reference
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    //mutable reference
    change(&mut s1);
    println!("the str is {}", s1);
    //只能有一个可变引用，或者多个不可变引用
}

//borrow: 引用作为函数的参数的行为叫做借用
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}
