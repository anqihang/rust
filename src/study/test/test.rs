#[cfg(test)]
pub mod tests{
    #[test]
    pub fn it_works(){
        assert_eq!(2 + 2, 4);// 失败会调用panic!，成功什么也不发生
    }
    #[test]
    pub fn another(){
        panic!("this is panic message");
    }
}