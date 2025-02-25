fn main(){
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let v1_into_iter = v1.into_iter();
    let mut v2 = vec![1,2,3];
    let v2_iter_mut = v2.iter_mut();
    for val in v1_iter{
        println!("{}",val);
    }
}