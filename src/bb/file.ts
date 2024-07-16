/**
 * 文件操作.
 */

import { dirname, join } from "@std/path";
import { TextLineStream } from "@std/streams";

/**
 * 创建目录.
 *
 * mkdir -p
 */
export async function 建目录(路径: string): Promise<void> {
  await Deno.mkdir(路径, { recursive: true });
}

/**
 * 创建上级目录.
 */
export async function 建目录1(路径: string): Promise<void> {
  const p = dirname(路径);
  await 建目录(p);
}

/**
 * 目录中的一项 (用于 列出).
 */
export interface 目录项 {
  /**
   * 类型.
   *
   * d: 目录.
   * f: 普通文件.
   */
  t: "d" | "f";

  /**
   * 名称
   */
  n: string;

  /**
   * 路径
   */
  p: string;
}

/**
 * 列出目录的内容.
 *
 * ls
 */
export async function 列出(路径: string): Promise<Array<目录项>> {
  const o = [] as Array<目录项>;
  for await (const i of Deno.readDir(路径)) {
    if (i.isFile) {
      o.push({
        t: "f",
        n: i.name,
        p: join(路径, i.name),
      });
    } else if (i.isDirectory) {
      o.push({
        t: "d",
        n: i.name,
        p: join(路径, i.name),
      });
    }
  }
  return o;
}

export interface 首尾项 extends 目录项 {
  /**
   * 中间的部分.
   */
  m: string;
}

/**
 * 列出目录内容, 根据首尾过滤.
 */
export async function 列出_首尾(
  路径: string,
  首: string,
  尾: string,
): Promise<Array<首尾项>> {
  const o = [] as Array<首尾项>;
  for (const i of await 列出(路径)) {
    if (i.n.startsWith(首) && i.n.endsWith(尾)) {
      o.push({
        m: i.n.slice(首.length, i.n.length - 尾.length),
        ...i,
      });
    }
  }
  return o;
}

/**
 * 按行读取文本文件.
 */
export async function 读取文本行(文件: string): Promise<Array<string>> {
  using f = await Deno.open(文件);
  return await Array.fromAsync(
    f.readable.pipeThrough(new TextDecoderStream()).pipeThrough(
      new TextLineStream(),
    ),
  );
}
