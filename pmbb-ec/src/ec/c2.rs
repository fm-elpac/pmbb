//! 类型 2 (EC type 2): 盘间冗余
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use pm_bin::log::debug;

use super::{
    EC元数据, EC元数据_EC1, EC元数据_文件, EC参数, 保存_ec元数据, 写入sha256, 后缀_SHA256,
    相对路径, 获取_ec, E, EC, L,
};

/// 命令 pmbb-ec c2
pub fn c_c2(a: Vec<String>) -> Result<(), Box<dyn Error>> {
    let ec = 获取_ec("RS_1_0_4MB")?;
    debug!("PMBB_EC={:?}", ec);

    // TODO 检查命令行参数
    // 解析命令行参数
    let 输出目录 = a[0].clone();
    let 元数据文件名 = a[1].clone();
    let mut 原始文件名: Vec<String> = Vec::new();
    let mut 冗余文件名: Vec<String> = Vec::new();
    let j = 2 + ec.原始块数 as usize;
    for i in 2..j {
        原始文件名.push(a[i].clone());
    }
    let k = 2 + ec.原始块数 as usize + ec.冗余块数 as usize;
    for i in j..k {
        冗余文件名.push(a[i].clone());
    }

    // 元数据文件路径
    let 元: PathBuf = [&输出目录, &元数据文件名].iter().collect();
    debug!("  {:?}", 元);
    // 构建元数据
    let mut m = EC元数据::default();
    m.pmbb_ec = ec.原始.clone();
    // 读取原始数据文件 (4MB 块大小)
    let 读 = 文件读取器::new(原始文件名.clone(), L)?;
    for i in 原始文件名 {
        m.file.push(EC元数据_文件 {
            p: 相对路径(&输出目录, &i)?,
            b: 0,
            sha256: None,
            ec: None,
        });
    }
    // 输出冗余数据文件
    let mut 写: Vec<写入sha256> = Vec::new();
    for i in 冗余文件名 {
        // _sha256.txt 文件
        let f: String = PathBuf::from(&i)
            .file_name()
            .ok_or_else(|| E::new(format!("bad filename {}", &i)))?
            .to_string_lossy()
            .to_string();
        let s: PathBuf = [&输出目录, &format!("{}{}", &f, 后缀_SHA256)]
            .iter()
            .collect();
        let sp: String = s.to_string_lossy().to_string();
        写.push(写入sha256::new(&i, &sp)?);
        m.file.push(EC元数据_文件 {
            p: 相对路径(&输出目录, &i)?,
            b: 0,
            sha256: Some(相对路径(&输出目录, &sp)?),
            ec: Some(EC元数据_EC1),
        });
    }

    // 准备工作完毕, 正式开始计算
    let b = 计算_ec(ec, 读, 写)?;
    // 更新文件长度
    for i in 0..b.len() {
        m.file[i].b = b[i];
    }

    // 最后写入元数据
    保存_ec元数据(&元.to_string_lossy(), &mut m)?;
    Ok(())
}

/// 读取多个文件, 按照块大小, 如果不足, 填 0
struct 文件读取器 {
    /// 打开的文件, 用来读取数据, 如果为 None, 表示读取完毕
    f: Vec<Option<File>>,
    /// 块长
    s: usize,
    /// 读取字节数
    b: Vec<usize>,
}

impl 文件读取器 {
    /// 打开文件
    pub fn new(p: Vec<String>, s: usize) -> Result<Self, Box<dyn Error>> {
        let mut f = Vec::new();
        for i in p {
            debug!("read {}", i);
            f.push(Some(File::open(i)?));
        }
        let l = f.len();
        Ok(Self {
            f,
            s,
            b: vec![0; l],
        })
    }

    /// 所有文件全部读取一块, 如果全部读取完毕, 返回 None
    pub fn 读(&mut self) -> Result<Option<Vec<Vec<u8>>>, Box<dyn Error>> {
        if self.f.iter().any(|i| i.is_some()) {
            let mut o = Vec::new();
            for i in 0..self.f.len() {
                // 读取缓冲区 (4MB)
                let mut b: Vec<u8> = vec![0; self.s];
                if let Some(f) = &mut self.f[i] {
                    let l = f.read(&mut b)?;
                    if 0 == l {
                        // 文件读取完毕
                        self.f[i] = None;
                    } else {
                        // 读取字节计数
                        self.b[i] += l;
                    }
                }
                o.push(b);
            }
            Ok(Some(o))
        } else {
            Ok(None)
        }
    }

    /// 返回读取长度
    pub fn 长度(&self) -> Vec<usize> {
        self.b.clone()
    }
}

/// 计算 EC_RS 生成冗余文件
fn 计算_ec(
    ec: EC参数,
    mut r: 文件读取器,
    mut w: Vec<写入sha256>,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let c = EC::new(ec.clone())?;
    // 循环计数, 用于 debug
    let mut x = 0;

    loop {
        debug!("  {}", x);
        // 读取数据
        match r.读()? {
            Some(mut b) => {
                // 空白数据
                for _ in 0..ec.冗余块数 {
                    b.push(vec![0; ec.块长]);
                }
                // 计算 EC
                c.编码(&mut b)?;

                // 保存结果
                for i in 0..ec.冗余块数 as usize {
                    w[i].写(&b[i + ec.原始块数 as usize])?;
                }
            }
            None => {
                // 计算完毕
                break;
            }
        }

        // 4MB
        x += 4;
    }
    // 结束清理
    for i in w {
        i.close()?;
    }
    Ok(r.长度())
}

/// 命令 pmbb-ec r2
pub fn c_r2(a: Vec<String>) -> Result<(), Box<dyn Error>> {
    // TODO
    println!("c_r2 {:?}", a);
    Ok(())
}

// TODO
