fn main(){
  let mut post = Post::new():
  post.add_text("hello world");
  println!("post: {}", post.content());

  post.request_review();
  println!("post: {}", post.content());

  post.approve();
  println!("post: {}", post.content());
}

struct Post{
    state:Option<Box<dyn State>>
    content:String,
}
impl Post{
    fn new()->Post{
        Post{
            state:Some(Box::new(Draft {})),
            content:String::new(),
        }
    }
    fn add_text(&mut self,text:&str){
        self.content.push_str(text);
    }
    fn content(&self)->&str{
        self.state.as_ref().unwrap().content(&self.content)
    }
    fn request_review(&mut self){
        if let Some(s)  = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    fn approve(&mut self){
        if let Some(s)= self.state.take(){
            self.state = Some(s.approve());
        }
    }
}
trait State {
    fn request_review(self:Box<Self>)-> Box<dyn State>;
    fn approve(self:Box<Self>)-> Box<dyn State>;
    fn content<'a>(&self,post:&'a Post)->&'a str{
        "" // 默认实现，不需要Draft和PendingReview实现了
    }
}
trait Draft {}
impl State for Draft{
    fn request_review(self:Box<Self>)-> Box<dyn State>{
        Box::new(PendingReview {})
    }
    fn approve(self:Box<Self>)-> Box<dyn State>{
        self
    }
}
trait PendingReview {}
impl State for PendingReview{
    fn request_review(self:Box<Self>) -> Box<dyn State>{
        self
    }
    fn approve(self:Box<Self>) -> Box<dyn State>{
        Box::new(Published {})
    }
}
trait Published {}
impl State for  Published{
    fn request_review(self:Box<Self>) -> Box<dyn State>{
        self
    }
    fn approve(self:Box<Self>) -> Box<dyn State>{
        self
    }
    fn content<'a>(&self,post:&'a Post) -> &'a str{
        &post.content
    }
}