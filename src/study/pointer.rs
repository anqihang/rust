pub mod pointer{
    pub fn main(){
        let b = Box::new(5);// 在堆上分配一个整数
        println!("b = {}", b);
		// let list = Cons(5,Cons(6,Cons(7,Nil)));// 无法确定大小
		let list = Cons(8,Box::new(Cons(9,Box::new(10,Nil))));

        //
        let x = 5;
        let y = &x;
        let z = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);// 解引用,直接获取堆中内存的值，而不是引用地址
        assert_eq!(5, *z);
    }
}
enum List{
	Cons(i32,List),
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