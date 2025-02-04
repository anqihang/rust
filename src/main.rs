/*
 * @Author: 安琦航 anqihang0106@outlook.com
 * @Date: 2025-01-29 15:18:24
 */
mod study;
use study::control as Control;// as 重命名
fn main(){
    study::variable::main();
    // control::main();
    Control::main()
}