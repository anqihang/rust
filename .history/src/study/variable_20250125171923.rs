pub fn main(){
    let x = 5; // 不可变
    println!("x is {}",x);
    // x=6;//! error
    // println!("x is {}",x);
    let mut y =5; // 可变
    println!("y is {}",y);
    y = 6;// 不可更改类型 y = "hello";//! error
    println!("y is {}",y);

    const ONE_HOUR:u32 =  60*60;// 常量
    // shadow
    let z = 1;
    let z = z+1;// 可更改类型
    {
        let z = z+2;
        println!("z is {}",z);// 4
    }
    println!("z is {}",z);// 2
}