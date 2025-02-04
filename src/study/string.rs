/*
 * @Author: 安琦航 anqihang0106@outlook.com
 * @Date: 2025-01-31 19:57:58
 */
pub fn main(){
    //! String 是Vec<u8>的封装
    let mut s = String::new();// 创建一个空字符串,
    s = "hello".to_string();// 使用to_string方法创建字符串
    let ss = String::from("hello");// 使用from方法创建字符串
    ss.push_str(", world!");// push_str方法附加字符串字面值
    s.push('w');// push方法附加单个字符
    let s0 = s+&ss;// 使用+运算符,此时s被移动到s0中,不能再使用 &String 被强制转换成了 &str
    let s1 = format!("{}-{}-{}",ss,s,ss);// 使用format!宏拼接字符串
    let a = "hello world";
    let b = a.chars();// 返回字符串的字符
    let c = a.bytes();// 返回字符串的字节
}