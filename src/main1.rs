mod util;

use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
use rand::Rng;


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    println!("Hello, world!");
    //
    // i8 u8/i16 u16/i32 u32/i64 u64/isize:自动类型推断(有符号位) usize
    //
    let x = 5;
    let x = x + 1;// 6
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");// 12
    }

    println!("The value of x is: {x}");// 6
    let tup = (1, true, 'a');// 元组
    println!("{}", tup.0);// 1
    let r = result(5);
    println!("{r}");// 15
    // 随机数
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..10);// [1,10)随机
    println!("{random_number}");
    // 控制
    if random_number > 5 {
        println!("{}", 'a');
    }
    let mut count = 0;// mut: 可修改
    'out: loop { // 无限循环
        count += 1;
        if count == 10 {
            let mut sec = count;
            let o = loop {
                sec -= 1;
                if sec == 2 {
                    break sec;
                }
            };
            println!("${o}"); // 2
            break 'out;// 打断循环
        }
    };
    let mut count = 10;
    while count > 0 {
        count -= 1;
    }

    for i in (0..10).rev() {
        println!("{i}!"); // 10 9 .. 0
    }
    //一个内存（所有权）只绑定一个变量
    {
        let s = "ownership";
    }
    // println!("{s}");// s 不存在
    {
        let mut s = String::from("Hello");// 分配堆上的内存
        s.push_str("Word!");
        let ss = s;// s将无效，s被move到ss
        let sss = ss.clone();// ss 有效, ss深拷贝到sss
        move1(sss);// sss被move到函数里了，sss将无效
    }
    // s不存在
    // reference引用
    let a = String::from("Hello Word!");
    let a = "Hello Word!".to_string();// 相同 String::from
    let l = len(&a);// 传入的是引用，不改变所有权
    let mut mua = String::from("Hello");
    let new_str = add_length(&mut mua);// 传入引用，可修改
    // slice
    let word = &a[6..];// word
    let hello = &a[..5];// hello
    let hello_word = &a[..];
    let space = &a[5..6];// ' '
    let arr = [1, 2, 3];
    let two = &arr[1..2];// 2
    // 结构体
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let mut user1 = User {
        active: false,
        username: String::from("anqihang"),
        email: String::from("1711404616@qq.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anqihang@qq.com");
    let user2 = User {
        active: true,
        ..user1
    };
    // 元组结构体
    struct Color(i32, i32, i32);

    let rectangle = Rectangle {
        width: 100,
        height: 20,
    };
    area(&rectangle);
    println!("{:?}", rectangle);
    dbg!(&rectangle);
    rectangle.area();
    // 6
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let four = IpAddrKind::V6(String::from("127.0.0.1"));
    let home = IpAddrKind::V4(127, 0, 0, 1);
    enum Message {
        Quit,
        Move { x: u32, y: u32 },
        Write(String),
    }
    impl Message {
        fn call(&self) {
            println!("{self}")
        }
    }
    let null: Option<u32> = None;// null 值
    let may_null: Option<i8> = Some(8);
    enum Coin {
        Ten,
        Five,
        Print(String),
    }
    fn value_in_coin(coin: Coin) -> u8 {
        match coin {
            Coin::Ten => 10,
            Coin::Five => {
                5
            }
            other => {
                println!("{other}");
                8
            }// _=>()
        }
    }
    let five = Coin::Five;
    if let Coin::Five = five {} else {}

    //
    let count = util::math::add(1, 2);
    //vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    let mut v = vec![1, 2, 3];
    let third: &i32 = &v[2];// 3
    let second: Option<&i32> = v.get(1);// 2
    for i in &mut v {
        *i += 4; // *解析i
    }
    // map
    let mut scores = HashMap::new();
    enum Key {
        String,
        Boolean,
        I32,
    }
    scores.insert(Key::String, 1);

    // 错误
    // panic!("终止！");
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("{:#?}", error)
            },
            other => {
                panic!("{:#?}", other);
            }
        }
    };
    let un_wrap = File::open("hello.txt").unwrap();
    let expect = File::open("hello.txt").expect("hello.txt don't open successfully");
}

fn result(_x: i32) -> i32 {
    _x + 10// 无 ; 表返回
}

fn move1(str: String) {
    println!("{str}")
}

fn len(s: &String) -> usize {// &String s 就是引用 a
    s.len()
}

fn add_length(str: &mut String) {
    str.push_str("mut")
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
