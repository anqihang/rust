pub fn main() {
    let x = 5;
    if x > 0 {
        println!("x is positive");
    } else if x < 0 {
        println!("x is negative");
    }
    let y: char = if x > 0 { 'p' } else { 'n' }; // 返回相同的类型
    // loop while for
    let mut i = 0;
    'out: loop {
        println!("loop");
        if i == 1 {
            loop {
                i = i - 1;
                println!("inner loop");
                if i == 0 { break; }
            }
        }
        if i == 2 {
            continue;
        }
        if i == 3 {
            break 'out;
        }
    }
    let y = loop {
        if i == 2 {
            break i * 2;
        }
    }; // 4

    while i < 5 {
        i += 1;
        println!("while");
    }

    let arr = [1, 2, 3, 4, 5];
    for i in arr {
        println!("for {}", i);
    }
}