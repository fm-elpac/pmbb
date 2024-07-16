/**
 * 运行新的进程.
 */

import { log1 } from "./log.ts";

function 命令0(命令: Array<string>): string {
  return 命令[0];
}

function 命令1(命令: Array<string>): Array<string> {
  return 命令.slice(1);
}

async function 检查退出码(p: Deno.ChildProcess): Promise<number> {
  const { code } = await p.status;
  if (0 != code) {
    log1("  退出码 " + code);
  }
  return code;
}

/**
 * 执行一条命令.
 *
 * 命令: 命令行.
 * 返回: 退出码.
 */
export async function 运行(命令: Array<string>): Promise<number> {
  log1("  运行: " + 命令.join(" "));

  const c = new Deno.Command(命令0(命令), {
    args: 命令1(命令),
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  });
  const p = c.spawn();
  return await 检查退出码(p);
}

// pipe stdout
async function 输出文件(
  p: Deno.ChildProcess,
  文件: string,
): Promise<Deno.FsFile> {
  const o = await Deno.open(文件, {
    write: true,
    truncate: true,
    create: true,
  });
  p.stdout.pipeTo(o.writable);

  return o;
}

/**
 * 执行一条命令, 结果输出到文件.
 *
 * 命令 > 文件
 */
export async function 运行_输出(
  命令: Array<string>,
  文件: string,
): Promise<number> {
  log1("  运行: " + 命令.join(" ") + " > " + 文件);

  const c = new Deno.Command(命令0(命令), {
    args: 命令1(命令),
    stdin: "inherit",
    stdout: "piped",
    stderr: "inherit",
  });
  const p = c.spawn();

  using _o = await 输出文件(p, 文件);
  return await 检查退出码(p);
}

/**
 * 执行 2 条命令, 通过管道连接, 结果输出到文件.
 *
 * 命令1 | 命令2 > 文件
 */
export async function 运行_管道_输出(
  命令_1: Array<string>,
  命令_2: Array<string>,
  文件: string,
): Promise<Array<number>> {
  log1("  运行: " + 命令_1.join(" ") + " | " + 命令_2.join(" ") + " > " + 文件);

  const c1 = new Deno.Command(命令0(命令_1), {
    args: 命令1(命令_1),
    stdin: "inherit",
    stdout: "piped",
    stderr: "inherit",
  });
  const p1 = c1.spawn();

  const c2 = new Deno.Command(命令0(命令_2), {
    args: 命令1(命令_2),
    stdin: "piped",
    stdout: "piped",
    stderr: "inherit",
  });
  const p2 = c2.spawn();

  // 管道
  p1.stdout.pipeTo(p2.stdin);

  using _o = await 输出文件(p2, 文件);
  return [await 检查退出码(p1), await 检查退出码(p2)];
}
