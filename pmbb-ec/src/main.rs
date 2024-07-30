//! `pmbb-ec`: Use erasure code (EC) to recovery data.
//! <https://github.com/fm-elpac/pmbb>
//!
//! Command (and example):
//!
//! + `pmbb-ec sha256`: Calculate sha256 of data block (use this to verify data).
//!
//!   ```sh
//!   env PMBB_EC=RS_3_2_4MB pmbb-ec sha256 1.tar
//!   ```
//!
//! + `pmbb-ec t1`: EC test create (RS).
//!
//!   ```sh
//!   env PMBB_EC=RS_3_2_2KB pmbb-ec t1 1.bin 2.bin 3.bin 4.ec 5.ec
//!   ```
//!
//! + `pmbb-ec t2`: EC test recovery (RS).
//!
//!   ```sh
//!   env PMBB_EC=RS_3_2_2KB pmbb-ec t2 1.bin -2.bin -3.bin 4.ec 5.ec
//!   ```
//!
//! + `pmbb-ec c1`: Create recovery data (type 1).
//!
//!   ```sh
//!   env PMBB_EC=RS_20_1_2KB_4MB pmbb-ec c1 out_dir meta_pmbb_ec.json 1.tar 2.tar 3.tar 4.tar 5.ec
//!   ```
//!
//! + `pmbb-ec r1`: Do recovery (type 1).
//!
//!   ```sh
//!   env PMBB_EC=RS_20_1_2KB_4MB pmbb-ec r1 meta_pmbb_ec.json out_dir
//!   ```
//!
//! + `pmbb-ec c2`: Create recovery data (type 2).
//!
//!   ```sh
//!   env PMBB_EC=RS_3_2_4MB pmbb-ec c2 out_dir meta_pmbb_ec.json 1.tar 2.tar 3.tar 4.ec 5.ec
//!   ```
//!
//! + `pmbb-ec r2`: Do recovery (type 2).
//!
//!   ```sh
//!   env PMBB_EC=RS_3_2_4MB pmbb-ec r2 meta_pmbb_ec.json out_dir
//!   ```
//!
//! Env var:
//!
//! + `PMBB_EC`:
//!
//!   ```sh
//!   PMBB_EC=RS_3_2_4MB
//!   ```
//!
//! TODO
#![deny(unsafe_code)]

use std::process::ExitCode;

use pm_bin::{cli_arg, init_env_logger, pm_init};
pm_init!();

pub mod ec;

use ec::{c_c1, c_c2, c_r1, c_r2, c_sha256, c_t1, c_t2};

fn main() -> Result<(), ExitCode> {
    init_env_logger();

    if let Some(a) = cli_arg(print_version) {
        if a.len() > 0 {
            // 第 1 个参数: 命令
            let r: Vec<String> = (&a[1..]).into();
            match a[0].as_str() {
                "sha256" => c_sha256(r).unwrap(),
                "t1" => c_t1(r).unwrap(),
                "t2" => c_t2(r).unwrap(),

                "c1" => c_c1(r).unwrap(),
                "r1" => c_r1(r).unwrap(),

                "c2" => c_c2(r).unwrap(),
                "r2" => c_r2(r).unwrap(),

                _ => {
                    println!("ERROR: unknown command {}", a[0]);
                    return Err(ExitCode::from(1));
                }
            }
            Ok(())
        } else {
            println!("ERROR: unknown command");
            Err(ExitCode::from(1))
        }
    } else {
        Ok(())
    }
}
