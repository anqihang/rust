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

    pub fn week_reference(){
        let x = Rc::new(5);// 引用计数为1,在堆中
        // let y = Rc::clone(&x);// 引用计数为2,只有为0才能清理
        let z = Rc::downgrade(&x);// 降级为Weak,引用计数为1,只要强引用计数为0，弱引用无需为0就可被清理
        z.upgrade().is_some();// 升级为强引用,返回Option<Rc<T>>


        #[derive(Debug)]
        struct Node{
            value:i32,
            children:RefCell<Vec<Rc<Node>>>,
            parent:RefCell<Week<Node>>
        }
        let leaf = Rc::new(Node{value:3,children:RefCell::new(vec![])});
        let branch = Rc::new(Node{value:5,children:RefCell::new(vec![Rc::clone(&left)])});

        {
        //  子级的 parent 是 强引用，branch  的 children 是 弱引用。所以当 branch 被清理时，其子级也会被清理 ，leaf 被清理时，其 parent 不会被清理。
            let leaf = Rc::new(Node{value:5,children:RefCell::new(vec![]),parent:RefCell::new(Week::new())});
            let branch = Rc::new(Node{value:5,children:RefCell::new(vec![Rc::clone(&leaf)]),parent:RefCell::new(Week::new())});
            *leaf.parent.borrow_mut()  = Rc::downgrade(&branch);
        }
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