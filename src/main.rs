use rand::Rng;

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
    let l = len(&a);// 传入的是引用，不改变所有权
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
