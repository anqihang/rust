/*
 * @Author: 安琦航 anqihang0106@outlook.com
 * @Date: 2025-01-31 16:31:30
 */
pub mod module;// 从pack/module.rs中加载模块相当于
// mod pack{
    // pub mod module{}
// }
// 在问价夹下创建mod.rs文件相当于把文件夹声明为mod,不再需要创建同名的文件,然后在mod.rs中声明mod module;