/**
 * pmbb-iso: 处理 iso9660 文件系统.
 *
 * 命令行参数: 命令 参数
 * 栗子:
 *
 * deno run -A pmbb-iso.ts ls 1.iso
 */

import { ENV_PMBB_DEBUG, ENV_PMBB_SORT, log1, P_VERSION } from "../bb/mod.ts";
import { 显示结果, 结果排序, 解析iso } from "../bb/iso/parse.ts";

export async function pmbb_iso(a: Array<string>) {
  const debug = Deno.env.get(ENV_PMBB_DEBUG) == 1 as unknown as string;
  const 命令 = a[0];
  if (debug) {
    log1("pmbb-iso: " + P_VERSION);
  }

  if ("ls" == 命令) {
    const 结果 = await 解析iso(a[1], debug);
    if (Deno.env.get(ENV_PMBB_SORT) == 1 as unknown as string) {
      结果排序(结果);
    }
    结果.forEach(显示结果);
  } else {
    log1("错误: 未知命令 " + 命令);
    throw new Error("unknown command");
  }
}

if (import.meta.main) {
  pmbb_iso(Deno.args);
}
