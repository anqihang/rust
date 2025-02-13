/*
 * @Author: 安琦航 anqihang0106@outlook.com
 * @Date: 2025-01-31 20:26:06
 */
use std::collections::HashMap;
pub fn main(){
    let mut a:HashMap<String,i32> = HashMap::new();// 创建一个空HashMap,   **所有的键是相同的类型，所有的值是相同的类型
    a.insert(String::from("blue"),10);// 插入键值对

    let str = String::from("yellow");

    a.insert(str,50);// str被移动到HashMap中,在这之后不能再使用str

    a.get(String::from("blue"));// Option<&V> 返回键对应的值的引用

    a.entry(String::from("yellow")).or_insert(50);// 如果值不存在则插入,存在则返回值的可变引用
    //
    let v = Vector::new();
    let teams = vec![String::from("blue"),String::from("yellow")];
    let initial_scores = vec![10,50];
    let scores:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();// 使用zip方法创建HashMap
}