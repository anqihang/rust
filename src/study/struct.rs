/*
 * @Author: 安琦航 anqihang0106@outlook.com
 * @Date: 2025-01-29 21:16:34
 */
 struct Account{
    name:String,
    password:String,
    active:bool,
 }
pub fn main(){
    let mut account = Account{
        name:String::from("anqihang"),
        password:String::from("1234567890"),
        active:false,
    }
    account.active = true;
    let account1 = Account{
        name:String::from("newAccount");
        ..account,
    }// account 无效，因为password被移动到account1中了，非拷贝移动

    //tuple struct 元组结构体
    let point = Point(1,2);
    println!("{},{}",point.0,point[1]);
    println!("{:#?}",point);// 输出debug的格式 :? / :#? 
    dbg!(&point);// dbg 会返回所有权


    let rectangle = Rectangle::square(5);
}
#[derive(Debug)] // 显示开启debug输出 实现Debug接口
struct Point(i32,i32);
// 类单元结构体,类似 () 
struct AlwaysEqual;
struct Rectangle{
    width:u32,
    height:u32,
}
impl Rectangle{// 与Rectangle 关联的方法 // 可以有多个impl块
    fn area(&self)->u32{// &self 是 self:&Self 的简写 代替 rectangle:&Rectangle 是不可变的引用
        self.width * self.height
    }
    fn square(size:u32)->Rectangle{// 关联函数
        Rectangle{
            width:size,
            height:size,
        }
    }
}