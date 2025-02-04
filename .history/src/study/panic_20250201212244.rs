/*
 * @Author: 安琦航 anqihang0106@outlook.com
 * @Date: 2025-02-01 14:37:59
 */
pub fn main(){
    let f = File::open("hello.txt");
    let f = match f{
        Ok(file)=>file,
        Err(error)=>{
            panic!("Problem opening the file: {:?}",error)
        },
    }
}