/**
 * pmbb-box: 按容量对文件分装 (装箱).
 *
 * 命令行参数: 输入目录 (文件大小列表, du), 输出目录 (装箱计划, box-plan).
 * 栗子:
 *
 * deno run -A pmbb-box.ts tmp1 box1
 */

import {
  ENV_PMBB_BN,
  ENV_PMBB_BS,
  log1,
  P_VERSION,
  分装,
  解析大小,
  读取du,
  读取sha256,
} from "../bb/mod.ts";

export async function pmbb_box(a: Array<string>) {
  const 输入目录 = a[0];
  const 输出目录 = a[1];
  log1("pmbb-box: " + P_VERSION);

  // 读取 sha256, du 数据
  const sha256 = await 读取sha256(输入目录);
  const du = await 读取du(输入目录);

  // 生成箱子及容量
  let bs = 解析大小("22GB"); // 默认: BD-R 25GB 单张光盘容量
  let bn = 1;
  const bs1 = Deno.env.get(ENV_PMBB_BS);
  if (null != bs1) {
    bs = 解析大小(bs1);
  }
  const bn1 = Deno.env.get(ENV_PMBB_BN);
  if (null != bn1) {
    bn = Number.parseInt(bn1);
  }

  // 箱子列表
  const b = [];
  for (let i = 0; i < bn; i += 1) {
    b.push(bs);
  }

  await 分装({
    du: du.d,
    sha256: sha256.d,
    b,
    f_sha256: sha256.f,
    f_du: du.f,
    o: 输出目录,
  });
}

if (import.meta.main) {
  pmbb_box(Deno.args);
}
