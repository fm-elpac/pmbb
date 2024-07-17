/**
 * pmbb-gen: 根据装箱计划复制文件.
 *
 * 命令行参数: 装箱计划目录, 原始目录, 输出目录.
 * 栗子:
 *
 * deno run -A pmbb-gen.ts box1 raw1 out1
 */

import { dirname, join } from "@std/path";

import { log1, P_VERSION, 建目录, 读取装箱计划 } from "../bb/mod.ts";

export async function pmbb_gen(a: Array<string>) {
  const 计划目录 = a[0];
  const 原始目录 = a[1];
  const 输出目录 = a[2];
  log1("pmbb-gen: " + P_VERSION);

  // 读取装箱计划
  const p = await 读取装箱计划(计划目录);

  // mkdir 缓存
  const d = new Set<string>();
  // 复制每个文件
  for (const [_, 路径] of p.d) {
    const 至 = join(输出目录, 路径);
    // 创建上级目录
    const 上级 = dirname(至);
    if (!d.has(上级)) {
      console.log("mkdir -p " + 上级);
      await 建目录(上级);
      d.add(上级);
    }

    const 从 = join(原始目录, 路径);
    console.log("cp " + 从 + " " + 至);
    await Deno.copyFile(从, 至);
  }
}

if (import.meta.main) {
  pmbb_gen(Deno.args);
}
