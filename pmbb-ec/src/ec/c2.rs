//! 类型 2 (EC type 2): 盘间冗余
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{absolute, PathBuf};
use std::vec::IntoIter;

use pm_bin::log::debug;

use super::{
    EC元数据, EC元数据_EC1, EC元数据_文件, EC参数, 保存_ec元数据, 写入sha256, 写入行, 后缀_SHA256,
    相对路径, 获取_ec, 解析_ec, 计算sha256, 读取_ec元数据, 读取sha256, E, EC, L,
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
    let mut 读sha256 = Vec::new();
    for i in 原始文件名.clone() {
        let sp = 文件路径sha256(&输出目录, &i)?;
        读sha256.push(写入行::new(&sp)?);

        m.file.push(EC元数据_文件 {
            p: 相对路径(&输出目录, &i)?,
            b: 0,
            sha256: Some(相对路径(&输出目录, &sp)?),
            ec: None,
        });
    }
    let 读 = 文件读取器::new(原始文件名, L)?;
    // 输出冗余数据文件
    let mut 写: Vec<写入sha256> = Vec::new();
    for i in 冗余文件名 {
        let sp = 文件路径sha256(&输出目录, &i)?;
        写.push(写入sha256::new(&i, &sp)?);

        m.file.push(EC元数据_文件 {
            p: 相对路径(&输出目录, &i)?,
            b: 0,
            sha256: Some(相对路径(&输出目录, &sp)?),
            ec: Some(EC元数据_EC1),
        });
    }

    // 准备工作完毕, 正式开始计算
    let b = 计算_ec(ec, 读, 写, 读sha256)?;
    // 更新文件长度
    for i in 0..b.len() {
        m.file[i].b = b[i];
    }

    // 最后写入元数据
    保存_ec元数据(&元.to_string_lossy(), &mut m)?;
    Ok(())
}

/// 计算 _sha256.txt 文件的路径
fn 文件路径sha256(o: &str, i: &str) -> Result<String, Box<dyn Error>> {
    let f: String = PathBuf::from(i)
        .file_name()
        .ok_or_else(|| E::new(format!("bad filename {}", i)))?
        .to_string_lossy()
        .to_string();
    let s: PathBuf = [o, &format!("{}{}", &f, 后缀_SHA256)].iter().collect();
    Ok(s.to_string_lossy().to_string())
}

/// 读取多个文件, 按照块大小, 如果不足, 填 0
#[derive(Debug)]
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
    pub fn 读(
        &mut self,
        mut ha: Option<&mut Vec<String>>,
    ) -> Result<Option<Vec<Vec<u8>>>, Box<dyn Error>> {
        if self.f.iter().any(|i| i.is_some()) {
            let mut o = Vec::new();
            for i in 0..self.f.len() {
                // 读取缓冲区 (4MB)
                let mut b: Vec<u8> = vec![0; self.s];
                // 计算的 sha256
                let mut h: String = "".into();

                if let Some(f) = &mut self.f[i] {
                    let l = f.read(&mut b)?;
                    if 0 == l {
                        // 文件读取完毕
                        self.f[i] = None;
                    } else {
                        // 读取字节计数
                        self.b[i] += l;
                        // 计算 sha256
                        if let Some(_) = &mut ha {
                            h = 计算sha256(&b[0..l]);
                        }
                    }
                }
                // 保存 sha256 计算结果
                if let Some(ha) = &mut ha {
                    ha.push(h);
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
    mut h: Vec<写入行>,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let c = EC::new(ec.clone())?;
    // 循环计数, 用于 debug
    let mut x = 0;

    loop {
        debug!("  {}", x);
        // 读取计算 sha256
        let mut rh = Vec::new();
        // 读取数据
        match r.读(Some(&mut rh))? {
            Some(mut b) => {
                // 写入读取数据的 sha256
                for i in 0..h.len() {
                    let t = &rh[i];
                    if t.len() > 0 {
                        h[i].写(t)?;
                    }
                }

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
    // 解析命令行参数
    let 元数据文件名 = a[0].clone();
    let 输出目录 = a[1].clone();

    // 读取元数据
    debug!("  {}", &元数据文件名);
    let m = 读取_ec元数据(&元数据文件名)?;
    // 解析 PMBB_EC 参数
    let ec = 解析_ec(m.pmbb_ec.clone())?;

    // 读取数据
    let mut 读 = 文件读取2::new(&m, &元数据文件名)?;
    let c = EC::new(ec.clone())?;

    // 写入重建的数据
    let mut o = Vec::new();
    for i in 0..ec.原始块数 as usize {
        let p1 = &m.file[i].p;
        let n = PathBuf::from(p1)
            .file_name()
            .ok_or_else(|| E::new(format!("bad filename {}", p1)))?
            .to_string_lossy()
            .to_string();
        let p2: PathBuf = [&输出目录, &n].iter().collect();
        let p = p2.to_string_lossy().to_string();
        debug!("write {}", p);
        o.push(File::create(p)?);
    }

    // 循环计数, 用于 debug
    let mut x = 0;
    // 开始计算
    loop {
        debug!("  {}", x);
        match 读.读()? {
            Some(mut b) => {
                let d: Vec<Option<usize>> = b.iter().map(|i| i.as_ref().map(|i| i.len())).collect();
                debug!("{:?}", d);
                // 恢复数据
                c.恢复(&mut b)?;

                // 保存结果
                for i in 0..ec.原始块数 as usize {
                    let x = b[i].as_ref().ok_or_else(|| E::new(format!("bad EC")))?;
                    o[i].write_all(x)?;
                }
            }
            None => {
                break;
            }
        }
        // 4MB
        x += 4;
    }
    // 截短文件
    for i in 0..ec.原始块数 as usize {
        let b = m.file[i].b;
        debug!("  {}", b);
        o[i].set_len(b as u64)?;
    }
    Ok(())
}

/// 读取多个文件, 根据 sha256 检查是否损坏
#[derive(Debug)]
struct 文件读取2 {
    /// 读取每个文件的数据, 一次读取一块, 并计算 sha256
    读: 文件读取器,
    /// 读取之前计算好的 sha256, 用来对照文件是否损坏
    s: Vec<IntoIter<String>>,
}

impl 文件读取2 {
    /// 创建实例
    pub fn new(m: &EC元数据, mp: &str) -> Result<Self, Box<dyn Error>> {
        let mut 文件名: Vec<String> = Vec::new();
        let mut s = Vec::new();
        let mpp: PathBuf = absolute(PathBuf::from(mp))?
            .parent()
            .ok_or_else(|| E::new(format!("bad path")))?
            .into();

        for i in &m.file {
            let mut p = mpp.clone();
            p.push(&i.p);
            文件名.push(p.to_string_lossy().to_string());

            let s1 = i
                .sha256
                .as_ref()
                .ok_or_else(|| E::new(format!("no sha256 file")))?;
            let mut hp = mpp.clone();
            hp.push(&s1);
            let hps = hp.to_string_lossy().to_string();

            debug!("read {}", hps);
            let h = 读取sha256(&hps)?;
            s.push(h.into_iter());
        }
        let 读 = 文件读取器::new(文件名, L)?;

        Ok(Self { 读, s })
    }

    /// 所有文件, 每个读取一块, 并检查 sha256, 如果读取完毕, 或者损坏, 返回 None
    pub fn 读(&mut self) -> Result<Option<Vec<Option<Vec<u8>>>>, Box<dyn Error>> {
        // 现在读取的数据计算的 sha256
        let mut h1 = Vec::new();
        match self.读.读(Some(&mut h1))? {
            None => Ok(None),
            Some(d) => {
                // 之前的数据块计算的 sha256
                let h2: Vec<Option<String>> = self
                    .s
                    .iter_mut()
                    .map(|i| i.next().map(|i| i.clone()))
                    .collect();

                let mut o = Vec::new();
                for i in 0..d.len() {
                    // 条件 1: sha256 相等
                    // 条件 2: 文件读取完毕
                    if (Some(h1[i].clone()) == h2[i]) || (None == h2[i]) {
                        o.push(Some(d[i].clone()));
                    } else {
                        o.push(None);
                    }
                }
                Ok(Some(o))
            }
        }
    }
}
