//! 类型 1 (EC type 1): 盘内冗余
use std::error::Error;

/// 命令 pmbb-ec c1
pub fn c_c1(a: Vec<String>) -> Result<(), Box<dyn Error>> {
    // TODO
    println!("c_c1 {:?}", a);
    Ok(())
}

/// 命令 pmbb-ec r1
pub fn c_r1(a: Vec<String>) -> Result<(), Box<dyn Error>> {
    // TODO
    println!("c_r1 {:?}", a);
    Ok(())
}

// TODO
