/**
 * @file main.rs
 * @author lihuax
 * @version 0.1
 * @descritpion
 */

// fn main() {
//    enum IpAddr {
//        V4(u8, u8, u8, u8),
//        V6(String),
//    }
//
//    let home = IpAddr::V4(127, 0, 0, 1);
//
//    let loopback = IpAddr::V6(String::from("::1"));
//}

fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // 在这里定义方法体
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
