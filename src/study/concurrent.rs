pub fn main(){
    let handle =thread::spawn(||{
        for i in 1..10{
            println!("thread 1: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    })// 主线程 结束此线程有也结束

    for i in 11...15{
        println!("thread 2: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}