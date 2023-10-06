
/**
 * @file main.rs
 * @author lihuax
 * @version 0.1
 * @descritpion 所有权
 *
 */

fn main(){

    /*
     * 变量的所有权规则：
     * 1. 每一个值都有一个变量，这个变量是该值的所有者
     * 2. 每个值同时只能有一个所有者
     * 3. 当所有者超出作用域（scope）时，该值将被删除
     */
    {                      // s 在这里无效，它尚未声明
        let s = "hello";   // 从此处起，s 是有效的
        // 使用 s
        println!("{}",s);
    }                      // 此作用域已结束，s 不再有效
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`
    //1. move 移动
    let num1 = 5;
    let num2 = num1;
    println!("{} and, {}", num1, num2);
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{} and, moved", s2);

    /*
     * 所有权与函数：
     * 将值传递给函数与给变量赋值的原理相似
     */
    let str1 = String::from("hello");
    takes_ownership(str1);
    //这里str1不再有效
    let num = 5;
    takes_copy(num);
    println!("{}", num);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn takes_copy(some_int: i32){
    println!("{}", some_int);
}
