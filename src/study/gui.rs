pub trait Draw{
    fn draw(&self);
}
pub struct Screen{
    pub components:vec!<Box<dyn Draw>>;
}
impl Screen{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}
pub struct Button{
    pub width:u32,
    pub height:u32,
    pub label:String
}
impl Draw for Button{
    fn draw(&self){}
}

pub fn main(){
    let screen = Screen{
        components:vec![Box::new(Button{width:100,height:50,label:String::from("Hello World")})]
    }
    screen.run();
}