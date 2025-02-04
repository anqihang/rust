/*
 * @Author: 安琦航 anqihang0106@outlook.com
 * @Date: 2025-01-30 16:21:15
 */
pub fn main(){
     let four = IpAddressKind::v4;
     let six = IpAddressKind::v6;
}
// enum IpAddressKind {
//     v4,v6
// }
// struct IpAddr{
//     kind: IpAddressKind,
//     address: String
// }
// let home = IpAddr{
//     kind: IpAddressKind::v4,
//     address: String::from("127.0.0.1")
// }
// let loopback = IpAddr{
//     kind:IpAddressKind::v6,
//     address: String::from("::1")
// }
enum IpAddr {
    v4(u8,u8,u8,u8),// 枚举可以包含任意类型的数据
    v6(String),
}
impl IpAddr{
    fn call(&self){
        
    }
}
let home = IpAddr::v4(127,0,0,1);
let loopback = IpAddr::v6(String::from("::1"));

// option 
let x:i8 = 5;
let y:Option<i8> = Some(6);
let none:Option<i8> = None;
// let z = x+y;// error

// match 
match(ip:IpAddr){
    IpAddr::v4=> {
        println!("{}",1);
        1
    },
    IpAddr::v6(address)=> {
        println!("{}",address);
    },
    // other=>other
    _=>0// _ 匹配所有情况并不使用值
}