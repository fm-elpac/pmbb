/**
 * 读取数据.
 */

import { log1 } from "./log.ts";
import { 列出_首尾, 读取文本行, 首尾项 } from "./file.ts";

/**
 * 读取 `du` 命令输出的结果.
 */
export function 解析du(行: Array<string>): Array<[number, string]> {
  const o = [] as Array<[number, string]>;
  for (const i of 行) {
    // 忽略空白行
    if (i.trim().length < 1) {
      continue;
    }

    // 分隔符: tab (\t)
    const S = "\t";
    // 格式: 字节数 文件路径
    const a = i.indexOf(S);

    o.push([Number.parseInt(i.slice(0, a)), i.slice(a + S.length, i.length)]);
  }
  return o;
}

/**
 * 读取 `sha256sum` 命令输出的结果.
 */
export function 解析sha256(行: Array<string>): Array<[string, string]> {
  const o = [] as Array<[string, string]>;
  for (const i of 行) {
    // 忽略空白行
    if (i.trim().length < 1) {
      continue;
    }

    // 分隔符: 空格
    const S = "  ";
    // 格式: sha256  文件路径
    const a = i.indexOf(S);

    o.push([i.slice(0, a), i.slice(a + S.length, i.length)]);
  }
  return o;
}

/**
 * 自动寻找 du 数据文件, 并读取.
 */
export async function 读取du(目录: string): Promise<{
  f: 首尾项;
  d: Array<[number, string]>;
}> {
  const l = (await 列出_首尾(目录, "du-", ".txt")).filter((i) => "f" == i.t);
  // 选择第一项
  const f = l[0];
  log1("  读取: " + f.p);

  const 行 = await 读取文本行(f.p);
  return {
    f,
    d: 解析du(行),
  };
}

/**
 * 自动寻找 sha256 数据文件, 并读取.
 */
export async function 读取sha256(目录: string): Promise<{
  f: 首尾项;
  d: Array<[string, string]>;
}> {
  const l = (await 列出_首尾(目录, "sha256-", ".txt")).filter((i) =>
    "f" == i.t
  );
  // 选择第一项
  const f = l[0];
  log1("  读取: " + f.p);

  const 行 = await 读取文本行(f.p);
  return {
    f,
    d: 解析sha256(行),
  };
}
