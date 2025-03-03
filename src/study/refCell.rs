pub trait Messenger{
    fn send(&self,msg:&str);
}
pub  struct LimitTracker('a,T:Messenger){
    messenger:&'a T,
    value:u32,
    max:u32
}
impl <'a,T> LimitTracker<'a,T>
    where T:Messenger{
        pub fn new(messenger:&'a T,max:u32)->LimitTracker<'a,T>{
            LimitTracker{
                messenger:&'a  T,
                value:0,
                max
            }
        }
    }
pub fn main(){

}