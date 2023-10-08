fn main() {
    let _v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    println!("v = {:?}", v);
    v.push(4);
    println!("v = {:?}", v);

    //let third: &i32 = &v[2];
    //println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &mut v{
        *i += 50;
    }

    for i in &v{
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}