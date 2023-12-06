use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

// fn parse_config(args: &[String]) -> Config // 连着返回了两次字符串切片，都是引用，所以不报错
// {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

impl Config {                  // OK分支是config类型，ERR分支是str
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments !!!");
        }
        let query = args[1].clone( );
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}