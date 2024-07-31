//! pmbb-ec 的主要功能
use std::error::Error;
use std::fs::{read, write};
use std::io::Read;

use pm_bin::log::debug;

mod c1;
mod c2;
mod err;
mod t;
mod util;

pub use c1::{c_c1, c_r1};
pub use c2::{c_c2, c_r2};
pub use err::E;
pub use t::{
    EC元数据, EC元数据_EC1, EC元数据_文件, EC参数, 后缀_SHA256, ENV_PMBB_EC, L, L2, P_VERSION,
};
pub use util::{
    file_or_stdin, Read1, 保存_ec元数据, 写入sha256, 写入行, 相对路径, 获取_ec, 解析_ec,
    计算sha256, 读取_ec元数据, 读取sha256, EC,
};

/// 命令 pmbb-ec sha256
pub fn c_sha256(a: Vec<String>) -> Result<(), Box<dyn Error>> {
    let ec = 获取_ec("RS_1_0_4MB")?;
    debug!("PMBB_EC={:?}", ec);

    let 文件名 = a[0].clone();
    debug!("文件名: {}", 文件名);

    let mut r = file_or_stdin(文件名)?;
    // 一次读取并处理一块数据
    loop {
        // 读取缓冲区
        let mut b: Vec<u8> = vec![0; ec.块长];
        let l = r.read(&mut b)?;
        if 0 == l {
            break;
        }

        let h = 计算sha256(&b[0..l]);
        println!("{}", h);
    }
    Ok(())
}

/// 命令 pmbb-ec t1
pub fn c_t1(a: Vec<String>) -> Result<(), Box<dyn Error>> {
    let ec = 获取_ec("RS_3_2_2KB")?;
    debug!("PMBB_EC={:?}", ec);
    let c = EC::new(ec.clone())?;

    // 检查命令行参数
    if a.len() != (ec.原始块数 + ec.冗余块数) as usize {
        return Err(Box::new(E::new(format!("bad CLI ({})", a.len()))));
    }
    // 读取并准备数据
    let mut b: Vec<Vec<u8>> = Vec::new();
    for i in 0..ec.原始块数 as usize {
        let 文件 = &a[i];
        debug!("read {}", 文件);
        let d = read(文件)?;
        b.push(d);
    }
    // 空白数据
    for _ in 0..ec.冗余块数 {
        b.push(vec![0; ec.块长]);
    }

    // 计算 EC
    c.编码(&mut b)?;

    // 保存结果
    for i in 0..ec.冗余块数 as usize {
        let d = &b[i + ec.原始块数 as usize];
        let 文件 = &a[i + ec.原始块数 as usize];
        debug!("write {}", 文件);
        write(文件, d)?;
    }
    Ok(())
}

/// 文件状态
#[derive(Debug, Clone)]
enum 文件名 {
    正常(String),
    缺失(String),
}

impl From<文件名> for String {
    fn from(v: 文件名) -> Self {
        match v {
            文件名::正常(s) => s,
            文件名::缺失(s) => s,
        }
    }
}

impl From<&String> for 文件名 {
    fn from(v: &String) -> Self {
        match v.strip_prefix("-") {
            Some(s) => Self::缺失(s.into()),
            None => Self::正常(v.into()),
        }
    }
}

/// 命令 pmbb-ec t2
pub fn c_t2(a: Vec<String>) -> Result<(), Box<dyn Error>> {
    let ec = 获取_ec("RS_3_2_2KB")?;
    debug!("PMBB_EC={:?}", ec);
    let c = EC::new(ec.clone())?;

    // 检查命令行参数
    if a.len() != (ec.原始块数 + ec.冗余块数) as usize {
        return Err(Box::new(E::new(format!("bad CLI ({})", a.len()))));
    }
    // 检查并处理文件名
    let f: Vec<文件名> = a.iter().map(From::from).collect();
    // 读取并准备数据
    let mut b: Vec<Option<Vec<u8>>> = Vec::new();
    for i in f.clone() {
        match i {
            文件名::正常(n) => {
                debug!("read {}", n);
                let d = read(n)?;
                b.push(Some(d));
            }
            文件名::缺失(_) => {
                b.push(None);
            }
        }
    }
    // 恢复数据
    c.恢复(&mut b)?;

    // 保存结果
    for i in 0..f.len() {
        match &f[i] {
            文件名::缺失(n) => {
                debug!("write {}", n);
                write(n, b[i].as_ref().unwrap())?
            }
            _ => {}
        }
    }
    Ok(())
}
