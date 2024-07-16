/**
 * pmbb-same: 查找重复的文件, 根据 sha256.
 *
 * 命令行参数: 输入目录.
 * 栗子:
 *
 * deno run -A pmbb-same.ts tmp1
 */

import { log1, P_VERSION, 读取sha256 } from "../bb/mod.ts";

function 分析(d: Array<[string, string]>) {
  console.log("文件总数 " + d.length);

  // 根据 sha256 分组
  const m = new Map<string, Array<string>>();
  for (const [s, p] of d) {
    if (m.has(s)) {
      m.get(s)!.push(p);
    } else {
      m.set(s, [p]);
    }
  }

  // 对结果进行计数
  let 唯一文件数 = 0;
  let 重复文件数1 = 0;
  let 重复文件数2 = 0;
  const 重复文件 = [] as Array<[string, Array<string>]>;
  for (const [k, v] of m.entries()) {
    if (1 == v.length) {
      唯一文件数 += 1;
    } else {
      重复文件数1 += v.length;
      重复文件数2 += 1;
      重复文件.push([k, v]);
    }
  }

  // 输出结果
  console.log("  唯一文件数 " + 唯一文件数);
  console.log("  重复文件数 " + 重复文件数1 + " (" + 重复文件数2 + ")");

  for (const [k, v] of 重复文件) {
    console.log("\n" + k + " (" + v.length + ")");
    for (const j of v) {
      console.log("  " + j);
    }
  }
}

export async function pmbb_same(a: Array<string>) {
  const 输入目录 = a[0];
  log1("pmbb-same: " + P_VERSION);

  // 读取 sha256 数据
  const r = await 读取sha256(输入目录);

  分析(r.d);
}

if (import.meta.main) {
  pmbb_same(Deno.args);
}
