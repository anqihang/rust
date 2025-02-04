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
}

struct Point(i32,i32);
// 类单元结构体,类似 () 
struct AlwaysEqual;