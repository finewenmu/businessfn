/// 业务函数库


pub mod arr; // 加载arr同名文件的模块

pub fn hello() {
    let i = arr::array_columns();
    println!("i num : {}", i);
}