use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
	
}
fn generate_workout(intensity: u32, random_number: u32) {
	// // 闭包,把代码存储在expensive_closure中，只执行一次，之后直接返回保存的值。 以 | 开始 num指定的参数 多个参数 ， 隔开
	// let expensive_closure = |num:u32|->u32 {// 单行可省略{, 类型标注可以省略
	// 	println!("calculating slowly...");
	// 	thread::sleep(Duration::from_secs(2));
	// 	num // 返回num
	// };
    //  let expensive_result =
    //     simulated_expensive_calculation(intensity);
 let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if  intensity < 25 {
        println!(
            "Today, do {} pushups!",
            // expensive_closure(intensity)// 传string参数， 闭包调用，返回值
			expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            // expensive_closure(intensity)// 传number参数 会报错，第一次调用闭包时，参数类型被锁进闭包中了，类型为string
			expensive_result.value(intensity)
        );
    }else{
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                // expensive_closure(intensity)
			expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
fn main() {
    let x = 4;

    let equal_to_x = move |z| z == x;// move会将x的所有权移动到闭包中

    let y = 4;

    assert!(equal_to_x(y));
}