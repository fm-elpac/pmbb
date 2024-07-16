/**
 * pmbb-scan: 使用 du 和 sha256 扫描数据目录.
 *
 * 命令行参数: 输入目录, 输出目录.
 * 栗子:
 *
 * deno run -A pmbb-scan.ts data1 out1
 */

import { join } from "@std/path";

import {
  log1,
  P_VERSION,
  建目录,
  时间标记,
  运行_管道_输出,
  运行_输出,
} from "../bb/mod.ts";

export async function pmbb_scan(a: Array<string>) {
  const 输入目录 = a[0];
  const 输出目录 = a[1];

  const 时间 = 时间标记(new Date());
  log1("pmbb-scan " + P_VERSION + ": " + 时间);

  log1("扫描目录: " + 输入目录);
  await 建目录(输出目录);

  // du -ab dir > o/du-T.txt
  await 运行_输出(
    ["du", "-ab", 输入目录],
    join(输出目录, "du-" + 时间 + ".txt"),
  );

  // find dir -type f -print0 | xargs -0 sha256sum > o/sha256-T.txt
  await 运行_管道_输出(
    ["find", 输入目录, "-type", "f", "-print0"],
    ["xargs", "-0", "sha256sum"],
    join(输出目录, "sha256-" + 时间 + ".txt"),
  );
}

if (import.meta.main) {
  pmbb_scan(Deno.args);
}
