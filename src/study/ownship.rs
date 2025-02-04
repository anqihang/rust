fn main(){
    // string
    let s = String::from("Hello, world!");
	s.pub_str("AX");
	let mut s2 = s.clone(); // s2 深拷贝 s。 s2->"Hello, world!"
	let s1=s;// s 被移动到 s1 指向同一块内存地址，s1 拥有所有权,s无效了。 s1->"Hello, world!"
	// println!("{}", s1);// error
	let len = get_len(s1); // s1所有权被转移到函数中，s1无效
	let len2 = get_str(&s1); // 指向s1的引用,不拥有值 &s1->s1->"Hello, world!"
	println!("{}",s1);
	change_str(&mut s2); // s2可修改，可变,可变引用同时只能有一个


	// string slice
	let str_s = &s2[0..5]; // s2的部分引用
	let str_first = &s2[..1];
	let str_end = &s2[1..];
	let str_new = &s2[..];
}
fn get_len(str:String)->usize{
	str.len()
}
fn get_str(str:&str)->usize{
	// str 无法修改， 不可变
	str.len()
}
fn change_str(str:&mut String){
	// str 可修改，可变
	str.push_str("AX");
}