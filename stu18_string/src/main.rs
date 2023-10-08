/**
 * rust 中的字符串
 * 1. rust 中的字符串是 UTF-8 编码的
 * 2. rust倾向于暴露可能的错误
 * 3. 字符串数据结构比开发者普遍认为的复杂得多
 */

fn main() {
    let data = "initial contents";
    let s1 = data.to_string();
    println!("{}", s1);
    let mut s2 = "🤣".to_string();
    s2.push_str("hahah");
    s2.push('a');
    println!("{}", s2);

    let s3 = String::from("Hello");
    let s4 = String::from("World");
    let s5 = s3 + &s4; // 注意 s3 被移动了，不能继续使用
    print!("{}", s5);
    println!("{}", s4);
    //print!("{}", s3);
    let str1 = &s5[0..2];
    println!("{}", str1);
}