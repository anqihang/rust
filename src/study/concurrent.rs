use std::thread;
use std::time::Duration;
pub mod concurrent {
    use std::thread;
    use std::time::Duration;
    use std::sync::mpsc;

    fn init() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("thread 1: {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        // 主线程 结束此线程有也结束

        handle.join().unwrap(); // 放在这里 线程先执行，执行完才执行主线程

        for i in 11..15 {
            println!("thread 2: {}", i);
            thread::sleep(Duration::from_millis(1));
        }

        // handle.join().unwrap();// 确保线程执行完主线程才结束，在这里线程和主线程同时执行
    }

    fn init_1() {
        let v = vec![1..7];
        let handle = thread::spawn(move || { // 2使用move获取v的所有权
            println!("{:?}", v);// ! error 1无法确定v的生命周期
        });
        handle.join().unwrap();
    }

    pub fn channel() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
    use std::sync::Mutex;
    pub fn mutex(){
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);// 6
    }
}