/**
 * @file main.rs
 * @author lihuax
 * @version 0.1
 * @descritpion learn slice
 */

fn main(){

    let s = String::from("Hello, world");
    let worldindex = first_word(&s);
    //let hello = &s[..5]; // 切片
    //let world: &str = &s[6..];
    //let whole: &str = &s[..];
    // attention:
    // 字符串切片的范围索引必须发生在有效的UTF-8字符边界内
    // 如果尝试从一个多字节的字符中创建字符串切片， 程序会报错退出
    println!("{}", worldindex);
}
//fn first_word(s: &String) -> usize{
//    let bytes = s.as_bytes();
//    for(i, &item) in bytes.iter().enumerate(){
//        if item == b' ' {
//            return i;
//        }
//    }
//    s.len()
//}
fn first_word(s: &str) -> &str{
    //如果有一个字符串 slice，可以直接传递它。
    //如果有一个 String，则可以传递整个 String 的 slice 或对 String 的引用。
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}