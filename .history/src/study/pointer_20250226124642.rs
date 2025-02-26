pub mod pointer{
    pub fn main(){
        let b = Box::new(5);// 在堆上分配一个整数
        println!("b = {}", b);
		// let list = Cons(5,Cons(6,Cons(7,Nil)));// 无法确定大小
		let list = Cons(8,Box::new(Cons(9,Box::new(10,Nil))));
    }
}
enum List{
	Cons(i32,List),
	Nil,
}