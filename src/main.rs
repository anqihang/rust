/*
 * @Author: 安琦航 anqihang0106@outlook.com
 * @Date: 2025-01-29 15:18:24
 */
mod study;
use study::concurrent;
// use study::control as Control;// as 重命名
// use study::trait1;
// use study::test::test::tests;
fn main(){
    // study::variable::main();
    // control::main();
    // Control::main();
    // trait1::main();
    // tests::it_works();
    concurrent::concurrent::channel();
}
