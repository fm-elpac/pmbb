/**
 * pmbb-iso: 处理 iso9660 文件系统.
 *
 * 命令行参数: 命令 参数
 * 栗子:
 *
 * deno run -A pmbb-iso.ts ls 1.iso
 */

import { log1, P_VERSION } from "../bb/mod.ts";
import { 解析iso } from "../bb/iso/parse.ts";

export async function pmbb_iso(a: Array<string>) {
  const 命令 = a[0];
  log1("pmbb-iso: " + P_VERSION);

  if ("ls" == 命令) {
    await 解析iso(a[1]);
  } else {
    log1("错误: 未知命令 " + 命令);
    throw new Error("unknown command");
  }
}

if (import.meta.main) {
  pmbb_iso(Deno.args);
}
