/*
 * @Author: 安琦航 anqihang0106@outlook.com
 * @Date: 2025-01-31 16:46:44
 */
pub fn main(){
    let v:Vec<i32> = Vec::new();// 向量，存储相同类型的元素,在推内存上是相邻存储的
    let mut v = vec![1,2,3];// 使用vec!宏来创建一个包含初始值的向量
    v.push(4);// 向
    // v[3];// 报错
    v.get(3);// Option<&i32> 返回索引位置的元素的引用
    for i in &v{
        println!("{}",i);
    }
}