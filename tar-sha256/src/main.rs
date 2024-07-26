#![deny(unsafe_code)]

use std::env;
use std::process::ExitCode;

use env_logger;

mod bin;
mod calc;

fn main() -> Result<(), ExitCode> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 命令行参数处理
    match env::args().skip(1).next() {
        Some(i) => match i.as_str() {
            "--version" | "--版本" => {
                bin::版本();
                Ok(())
            }
            _ => {
                calc::tar_sha256(i.into()).unwrap();
                Ok(())
            }
        },
        _ => {
            println!("bad CLI");
            Err(ExitCode::from(1))
        }
    }
}
