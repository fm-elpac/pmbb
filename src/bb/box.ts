/**
 * 按容量装箱.
 */

import { join, normalize, SEPARATOR } from "@std/path";

import { log1 } from "./log.ts";
import { 建目录, type 首尾项 } from "./file.ts";
import { 显示大小 } from "./size.ts";

/**
 * 将读取的原始 sha256 数据, 转换成方便处理的格式.
 *
 * 反向 sha256 映射: 路径 -> sha256
 */
function 转换sha256(d: Array<[string, string]>): Map<string, string> {
  const o = new Map<string, string>();
  for (const [s, p] of d) {
    o.set(normalize(p), s.trim());
  }
  return o;
}

/**
 * 树形 du 结构的单个节点
 */
interface 节点du {
  /**
   * 节点路径
   */
  p: string;
  /**
   * 节点名称 (路径的最后一部分)
   */
  n: string;

  /**
   * 节点总大小 (字节)
   */
  s: number;

  /**
   * 下级节点.
   *
   * null 表示普通文件, 否则表示目录.
   */
  c: null | Map<string, 节点du>;

  /**
   * 包含的文件个数
   */
  cn: number;
}

/**
 * 将读取的原始 du 数据, 转换成方便处理的格式.
 *
 * 构建树形 du 数据, 按照文件目录结构.
 */
function 转换du(
  d: Array<[number, string]>,
  sha256: Map<string, string>,
): 节点du {
  // 创建根节点
  const r: 节点du = {
    p: ".",
    n: ".",
    s: 0,
    c: new Map(),
    cn: 0,
  };
  // 将每个 du 项插入树
  for (const [s, p3] of d) {
    // 切分文件路径
    const p2 = normalize(p3);
    const p = p2.split(SEPARATOR).filter((i) => i.length > 0);
    const p1 = join(...p);

    // 当前节点
    let n = r;
    // 对文件路径的每一级进行查找
    for (const i of p.slice(0, p.length)) {
      const n1 = n.c!.get(i);
      if (null == n1) {
        // 创建新节点
        const n2 = {
          p: "",
          n: i,
          s: 0,
          c: new Map(),
          cn: 0,
        };
        n.c!.set(i, n2);
        // 更新当前节点
        n = n2;
      } else {
        // 更新当前节点
        n = n1;
      }
    }
    // 此时, 当前节点就是 du 项对应的节点, 更新大小
    n.p = p1;
    n.s = s;
  }

  // 检测空目录
  const 空目录 = [];

  // 检查叶节点 (普通文件), 移除下级
  function 递归检查(n: 节点du) {
    if ((null == n.c) || (0 == n.c.size)) {
      // 没有下级节点, 结束递归
      n.c = null;
      n.cn = 1;
    } else {
      // 检查每个下级节点
      for (const i of n.c.values()) {
        递归检查(i);
        // 检查空目录
        if ((null == i.c) && (!sha256.has(i.p))) {
          空目录.push(i);
          // 移除空目录
          n.c.delete(i.n);
        }
      }
      // 更新包含的文件个数
      n.cn = Array.from(n.c.values()).reduce((x, y) => x + y.cn, 0);
    }
  }

  递归检查(r);

  // TODO 空目录
  console.log("空目录 " + 空目录.length + " 个");

  // 更新根节点总大小
  r.s = Array.from(r.c!.values()).map((i) => i.s).reduce((x, y) => x + y, 0);
  return r;
}

function 分层贪心装箱(输入: 节点du, 箱: Array<number>): Array<节点du> {
  // 创建箱列表
  const o = [] as Array<节点du>;
  for (let i = 0; i < 箱.length; i += 1) {
    o.push({
      p: ".",
      // 箱的名称
      n: "box-" + (i + 1) + "_" + 箱.length,

      // 箱剩余容量 (字节)
      s: 箱[i],
      c: new Map(),
      cn: 0,
    });
  }

  // 剩余未装箱列表
  let 剩余 = [] as Array<节点du>;
  // 初始化填充剩余列表
  for (const i of 输入.c!.values()) {
    剩余.push(i);
  }

  // 无法装箱列表
  const 无法装箱 = [] as Array<节点du>;
  // 防止装箱死循环
  while ((无法装箱.length < 1) && (剩余.length > 0)) {
    // 对剩余列表按照从大到小排序 (降序)
    剩余.sort((a, b) => b.s - a.s);

    // 对每个箱进行贪心装箱
    for (const i of o) {
      // 存放本箱无法装下的东西
      const 暂存 = [] as Array<节点du>;
      // 对每个剩余项, 尝试装箱
      for (const j of 剩余) {
        if (j.s > i.s) {
          // 容量不足, 无法装箱
          暂存.push(j);
        } else {
          // 容量够, 可以装箱
          i.s -= j.s;
          i.c!.set(j.n, j);
        }
      }
      剩余 = 暂存;
    }

    // 处理剩余列表
    if (剩余.length > 0) {
      const 暂存 = [] as Array<节点du>;
      for (const i of 剩余) {
        if (null != i.c) {
          // 目录, 拆分 (分别处理每个下级)
          for (const j of i.c.values()) {
            暂存.push(j);
          }
        } else {
          // 普通文件, 无法装箱
          无法装箱.push(i);
        }
      }
      剩余 = 暂存;
    }
  }
  // 检查装箱失败
  if (无法装箱.length > 0) {
    log1("错误: 装箱失败 !");
    console.debug(无法装箱);

    throw new Error("box fail");
  }
  // 更新每箱的总大小
  for (const i of o) {
    const 下级 = Array.from(i.c!.values());
    i.s = 下级.reduce((x, y) => x + y.s, 0);
    i.cn = 下级.reduce((x, y) => x + y.cn, 0);
  }
  return o;
}

/**
 * 输出装箱清单
 */
function 打印装箱清单(o: Array<节点du>, 详细: boolean = false) {
  for (const i of o) {
    if (详细) {
      console.log("");
    }
    console.log(i.n + ": " + 显示大小(i.s) + " (" + i.cn + " 个文件)  " + i.s);
    if (详细) {
      const a = Array.from(i.c!.values());
      // 按照从大到小排序
      a.sort((x, y) => y.s - x.s);
      for (const j of a) {
        console.log("  " + 显示大小(j.s) + " (" + j.cn + ")  " + j.p);
      }
    }
  }
}

/**
 * 生成装箱计划
 */
async function 装箱计划(a: 装箱参数, sha256: Map<string, string>, 箱: 节点du) {
  // 输出目录
  const o = join(a.o, 箱.n);
  await 建目录(o);

  // 复制原始 sha256, du 文件
  await Deno.copyFile(a.f_sha256.p, join(o, a.f_sha256.n));
  await Deno.copyFile(a.f_du.p, join(o, a.f_du.n));

  // 装箱计划文件名
  const 文件名 = "bb_plan-" + 箱.n + "-" + a.f_sha256.m + "-sha256.txt";
  const f = join(o, 文件名);
  console.log("  " + f);

  // 装箱文件路径清单
  const 路径 = [] as Array<string>;

  function 递归检查(n: 节点du, o: Array<string>) {
    if (null == n.c) {
      // 普通文件
      if (sha256.has(n.p)) {
        o.push(n.p);
      }
    } else {
      // 目录文件
      for (const i of n.c.values()) {
        递归检查(i, o);
      }
    }
  }

  递归检查(箱, 路径);
  // 对路径排序
  路径.sort();

  // 生成装箱文件
  const 结果 = 路径.map((x) => sha256.get(x) + "  " + x);
  const 结果文本 = 结果.join("\n") + "\n";
  await Deno.writeTextFile(f, 结果文本);
}

export interface 装箱参数 {
  /**
   * 文件大小列表 (du)
   */
  du: Array<[number, string]>;

  /**
   * sha256 列表
   */
  sha256: Array<[string, string]>;

  /**
   * 每个箱子及容量
   */
  b: Array<number>;

  /**
   * 原始 sha256 文件
   */
  f_sha256: 首尾项;
  /**
   * 原始 du 文件
   */
  f_du: 首尾项;
  /**
   * 输出目录
   */
  o: string;
}

/**
 * 按照每个箱的容量限制, 对文件进行自动分层贪心分装.
 */
export async function 分装(a: 装箱参数) {
  log1("总文件数 " + a.sha256.length);
  log1("  目录数 " + (a.du.length - a.sha256.length));

  // 处理原始输入数据
  const sha256 = 转换sha256(a.sha256);
  const du = 转换du(a.du, sha256);

  // 检查装箱
  log1("需要装箱的文件总大小: " + 显示大小(du.s));
  const 箱总大小 = a.b.reduce((x, y) => x + y, 0);
  log1("  箱总数 " + a.b.length);
  log1("  箱大小: " + a.b.map(显示大小).join(", "));
  log1("  箱总大小: " + 显示大小(箱总大小));
  if (箱总大小 < du.s) {
    log1("错误: 装箱失败, 箱总容量太小 !");
    throw new Error("box err");
  }

  log1("开始计算装箱 .. .");
  const r = 分层贪心装箱(du, a.b);

  console.log("装箱结果:");
  打印装箱清单(r);

  // 创建输出目录
  await 建目录(a.o);
  console.log("");
  // 生成装箱计划
  for (const i of r) {
    await 装箱计划(a, sha256, i);
  }

  console.log("\n详细清单:");
  打印装箱清单(r, true);
}
