/*
 * @Author: 安琦航 anqihang0106@outlook.com
 * @Date: 2025-01-30 18:16:30
 */
use std::cmp::Ordering;
// use std::io;
use std::io::{self, Write};//等同于 use std::io use std::io::Write
use std::io::*;//引入所有
mod front_of_house{// 模块
    pub mod hosting{// 模块公有只允许访问模块，内容并非是公有的也需要加pub
        pub fn add(){}
    }
    mod serving{
        fn take_order(){
            super::hosting::add();// super 父级路径，相当与 ../
        }
    }
}
pub fn eat_at_restaurant(){
    crate::front_of_house::hosting::add();// 绝对路径
    front_of_house::hosting::add();// 相对路径
}
pub use front_of_house::hosting;// 重导入，可同时被外部的代码引用
// enum 默认是公有的

// crate - font_of_house 模块树
//          - hosting
//          - serving
//       - eat_at_restaurant