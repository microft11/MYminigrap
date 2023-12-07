use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // collect方法就是一个消费者适配器，使用它可以将一个迭代器中的元素收集到指定类型中

    // 对 build 返回的 `Result` 进行处理
    //  env::args 可以直接返回一个迭代器，再作为 Config::build 的参数传入
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(443);
    });

    println!("Searching for {}\n", config.query);
    println!("In file {}\n", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(443);
    }
}

