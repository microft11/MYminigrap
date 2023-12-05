use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // collect方法就是一个消费者适配器，使用它可以将一个迭代器中的元素收集到指定类型中
    // dbg!(args);
    // 两个变量来存储文件路径和待搜索的字符串
    let _query = &args[1];
    let file_path = &args[2];

    // let query: &String = &args[1];  // 所有权没有转移，还在args中
    // let file_path: &String = &args[2];

    // println!("Searching for {}", query);
    println!("In file {}\n", file_path);

    let contents = fs::read_to_string(&file_path)
        .expect("Should have able to read file");

    println!("{}", contents);
}
