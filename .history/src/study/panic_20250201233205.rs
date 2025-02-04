/*
 * @Author: 安琦航 anqihang0106@outlook.com
 * @Date: 2025-02-01 14:37:59
 */
 use std::fs::File;
pub fn main(){
    let f = File::open("hello.txt");
    let f = match f{
        Ok(file)=>file,
        Err(error)=>{
            match error.kind(){
                ErrorKind::NotFound=>match File::create("hello.txt"){
                    Ok(fc)=>fc,
                    Err(e)=>panic!("Problem creating the file: {:?}",e),
                },
                other_error=>panic!("Problem opening the file: {:?}",other_error)
            }
            panic!("Problem opening the file: {:?}",error)
        },
    }
    let     f =File::open("hello.txt").unwrap();// unwrap()方法会检查Result枚举的值，如果为Ok则返回Ok中的值，如果为Err则调用panic!宏
    let     f =File::open("hello.txt").expect("Failed to open hello.txt");// 和unwrap类似，但是可以自定义panic!的错误信息
    
    let mut f = File::open("hello.txt")?;// ?操作符会检查Result枚举的值，如果为Ok则返回Ok中的值，如果为Err则返可以连续调用,
}