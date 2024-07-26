// 解析 iso9660 文件列表.
//
// 参考资料: <https://wiki.osdev.org/ISO_9660>

import { 显示大小 } from "../size.ts";

// 光盘扇区大小
export const 扇区 = 2048;

// 读取文件的一部分数据
async function 读文件(
  f: Deno.FsFile,
  偏移: number,
  长度: number,
): Promise<[Uint8Array, number | null]> {
  await f.seek(偏移, Deno.SeekMode.Start);
  const b = new Uint8Array(长度);
  return [b, await f.read(b)];
}

// 读取一个光盘扇区
//
// 编号: 扇区编号
async function 读扇区(f: Deno.FsFile, 编号: number): Promise<Uint8Array> {
  const r = await 读文件(f, 编号 * 扇区, 扇区);
  // TODO 检查读取失败
  return r[0];
}

// 读取从某个扇区开始的数据
async function 读数据(
  f: Deno.FsFile,
  编号: number,
  长度: number,
): Promise<Uint8Array> {
  const r = await 读文件(f, 编号 * 扇区, 长度);
  // TODO 检查读取失败
  return r[0];
}

// 读取数据块的指定字节, 转换为文本
function 读文本(数据: Uint8Array, 偏移: number, 长度: number): string {
  const b = 数据.slice(偏移, 偏移 + 长度);
  const d = new TextDecoder();
  return d.decode(b);
}

// Joliet: UCS-2
function 读文本2(数据: Uint8Array, 偏移: number, 长度: number): string {
  const b = 数据.slice(偏移, 偏移 + 长度);
  const d = new TextDecoder("utf-16be");
  return d.decode(b);
}

function 读文本_2(
  数据: Uint8Array,
  偏移: number,
  长度: number,
  joliet: boolean = false,
): string {
  return joliet ? 读文本2(数据, 偏移, 长度) : 读文本(数据, 偏移, 长度);
}

export const 文件标志_目录 = 2;

// Directory entry, directory record
export interface 目录项 {
  // Length of Directory Record
  长度: number;
  // Extended Attribute Record length
  扩展属性长度: number;
  // Location of extent (LBA)
  位置: number;
  // Data length (size of extent)
  数据长度: number;

  // File flags
  文件标志: number;
  _目录: boolean;

  // File unit size for files recorded in interleaved mode
  交错模式文件单元大小: number;
  // Interleave gap size for files recorded in interleaved mode
  交错模式文件间隔大小: number;

  // Volume sequence number
  卷序号: number;
  // Length of file identifier (file name)
  文件名长度: number;
  // File identifier
  文件名: string;
  // 原始文件名
  _文件名?: Uint8Array;
  // 标记 . 和 .. 目录
  _?: boolean;
}

function 解析目录项(b: Uint8Array, joliet: boolean = false): 目录项 {
  const v = new DataView(b.buffer);
  const 文件标志 = b[25];
  const 文件名长度 = b[32];
  const 文件名 = 读文本_2(b, 33, 文件名长度, joliet);
  const _文件名 = b.slice(33, 33 + 文件名长度);

  return {
    长度: b[0],
    扩展属性长度: b[1],
    位置: v.getUint32(2, true),
    数据长度: v.getUint32(10, true),

    文件标志,
    _目录: (文件标志 & 文件标志_目录) != 0,

    交错模式文件单元大小: b[26],
    交错模式文件间隔大小: b[27],
    卷序号: v.getUint16(28, true),
    文件名长度,
    文件名,
    _文件名,
    // 检查 . 和 .. 目录
    _: (1 == 文件名长度) && ((0 == _文件名[0]) || (1 == _文件名[0])),
  };
}

// Primary Volume Descriptor
export interface 主卷描述符 {
  // System Identifier
  系统标识: string;
  // Volume Identifier
  卷标: string;
  // Volume Space Size
  卷空间块: number;
  // Volume Set Size
  逻辑卷集大小: number;
  // Volume Sequence Number
  逻辑卷集序号: number;
  // Logical Block Size
  逻辑块大小: number;

  // Directory entry for the root directory
  根目录: 目录项;
}

// Boot Record
export interface 启动记录 {
  // Boot System Identifier
  启动系统标识: string;
  // Boot Identifier
  启动标识: string;
}

// Volume Descriptor
export interface 卷描述符 {
  // Type
  类型: number;
  // Identifier
  标识: string;
  // Version
  版本: number;

  主?: 主卷描述符;
  启动?: 启动记录;
}

// 卷描述符类型代码 Volume Descriptor Type Codes
// Boot Record
export const 卷描述符类型_启动记录 = 0;
// Primary Volume Descriptor
export const 卷描述符类型_主卷描述符 = 1;
// Supplementary Volume Descriptor
export const 卷描述符类型_次卷描述符 = 2;
// Volume Partition Descriptor
export const 卷描述符类型_卷分区描述符 = 3;
// Volume Descriptor Set Terminator
export const 卷描述符类型_结束 = 255;

// 解析 Volume Descriptor
function 解析卷描述符(b: Uint8Array): 卷描述符 {
  const v = new DataView(b.buffer);
  const o: 卷描述符 = {
    类型: b[0],
    标识: 读文本(b, 1, 5),
    版本: b[6],
  };
  const joliet = 卷描述符类型_次卷描述符 == o.类型;

  switch (o.类型) {
    case 卷描述符类型_主卷描述符:
    case 卷描述符类型_次卷描述符:
      {
        o.主 = {
          系统标识: 读文本_2(b, 8, 32, joliet),
          卷标: 读文本_2(b, 40, 32, joliet),
          卷空间块: v.getUint32(80, true),
          逻辑卷集大小: v.getUint16(120, true),
          逻辑卷集序号: v.getUint16(124, true),
          逻辑块大小: v.getUint16(128, true),

          根目录: 解析目录项(b.slice(156, 156 + 34), joliet),
        };
      }
      break;
    case 卷描述符类型_启动记录:
      o.启动 = {
        启动系统标识: 读文本(b, 7, 32),
        启动标识: 读文本(b, 39, 32),
      };
      break;
  }
  return o;
}

// 递归遍历目录
async function 遍历目录(f: Deno.FsFile, 上级: 目录项, 路径: string) {
  // 防止死循环: 跳过 . 和 .. 目录
  if (上级._) {
    return;
  }
  const p = 路径 + (上级._目录 ? "/" : "");
  // 输出扇区编号 (数据长度) 和路径
  const 大小 = "(" + 显示大小(上级.数据长度) + " " + 上级.数据长度 + ")";
  console.log(上级.位置, 大小, p);
  // 如果不是目录, 结束递归
  if (!上级._目录) {
    return;
  }
  //console.log(上级);

  // 读取目录文件
  const b = await 读数据(f, 上级.位置, 上级.数据长度);

  // 当前目录项开始字节的位置
  let i = 0;
  // 循环解析每一个目录项
  while (i < b.length) {
    // 目录项长度
    const 长度 = b[i];
    // 单个目录项长度至少为 33 字节
    if (长度 > 33) {
      const 项 = 解析目录项(b.slice(i, i + 长度), true);
      // 递归遍历
      await 遍历目录(f, 项, 路径 + "/" + 项.文件名);
    } else if (0 == 长度) {
      // 当前目录解析完毕
      return;
    } else {
      // TODO
      console.log("长度 = " + 长度);
    }
    // 读取下一个目录项
    i += 长度;
  }
}

// 输入: 光盘镜像文件 (iso)
export async function 解析iso(文件名: string) {
  // 打开光盘镜像文件
  const f = await Deno.open(文件名);

  // 解析卷描述符, 从 16 扇区开始
  let vdi = 16;
  let vd继续 = true;
  // 保存根目录
  let 根目录: 目录项 | undefined;

  while (vd继续) {
    const 扇区 = await 读扇区(f, vdi);
    const vd = 解析卷描述符(扇区);
    // debug
    console.log(vdi, vd);

    if (卷描述符类型_结束 == vd.类型) {
      vd继续 = false;
    } else if (卷描述符类型_次卷描述符 == vd.类型) {
      根目录 = vd.主!.根目录;
    }
    // 继续读取下一个卷描述符
    vdi += 1;
  }

  if (null != 根目录) {
    // 消除根目录标记
    根目录._ = false;

    console.log("");
    // 从根目录开始, 遍历目录树
    await 遍历目录(f, 根目录, "");
  }
}
