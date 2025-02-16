pub fn main(){
    init(24,'a');
    let x = {
        let y=10;// 语句，无返回值
        y+1// 不带 ； 结尾是表达式，有返回值
    };// 11
    let z = five();
}
fn init(age:i32,name:char){
    println!("Hello, world!,{},{}",age,name);
}
fn five()->i32{
    5
}