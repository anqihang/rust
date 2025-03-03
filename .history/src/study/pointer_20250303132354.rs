use std::mem::drop;
use std::rc::Rc;
pub mod pointer{
    pub fn main(){
        let b = Box::new(5);// 在堆上分配一个整数
        println!("b = {}", b);
		// let list = Cons(5,Cons(6,Cons(7,Nil)));// 无法确定大小
		let list = Box::new(Cons(8,Box::new(Cons(9,Box::new(,Nil)))));

        //
        let x = 5;
        let y = &x;
        let z = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);// 解引用,直接获取堆中内存的值，而不是引用地址
        assert_eq!(5, *z);

        const D
        drop(d);// 手动清除数据
        println!("b = {}", b);

    }
    pub fn rc(){
         // 多引用指针，reference count Rc, 引用计数
        let list = Rc::new(Cons1(7,Rc::new(Cons1(8,Rc::new(Nil)))));
        println!("list = {:?}", Rc::strong_count(&list))
        let a = Cons1(6,Rc::clone(&list));// 只是增加引用计数，不会深拷贝数据
        println!("list = {:?}", Rc::strong_count(&list))
        let c = Cons1(5,Rc::clone(&list));
        println!("list = {:?}", Rc::strong_count(&list))
        {
            let d = Cons1(4,Rc::clone(&list));
            println!("list = {:?}", Rc::strong_count(&list))
        }
        // 计数减一
        println!("list = {:?}", Rc::strong_count(&list))
    }

    pub fn refCell(){
        
    }
}

enum List{
	Cons(i32,List),
	Nil,
}

enum List1{
    Cons1(i32,Rc<List>),
    Nil,
}

import std::ops::Deref;
struct MyBox<T>(T);
impl<T> Deref for MyBox {
    // fn new(x:T) -> Box<T>{
    //     Box::new(x)
    // }
    Type Target = T;
    fn deref(&self) -> &T{
        &self.0
    }
}