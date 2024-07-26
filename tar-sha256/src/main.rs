#![deny(unsafe_code)]

use std::process::ExitCode;

use pm_bin::{cli_arg, init_env_logger, pm_init};
pm_init!();

mod calc;

fn main() -> Result<(), ExitCode> {
    init_env_logger();

    if let Some(a) = cli_arg(print_version) {
        if a.len() != 1 {
            println!("bad CLI");
            Err(ExitCode::from(1))
        } else {
            calc::tar_sha256(a[0].clone()).unwrap();
            Ok(())
        }
    } else {
        Ok(())
    }
}
