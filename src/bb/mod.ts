/**
 * 胖喵贪吃 (PMBB) 主要代码 (库)
 */

export { ENV_PMBB_BN, ENV_PMBB_BS, P_VERSION } from "./conf.ts";

export { log1 } from "./log.ts";

export { 运行, 运行_管道_输出, 运行_输出 } from "./run.ts";

export { 时间标记 } from "./time.ts";

export { 显示大小, 解析大小 } from "./size.ts";

export type { 目录项, 首尾项 } from "./file.ts";
export { 列出, 列出_首尾, 建目录, 建目录1, 读取文本行 } from "./file.ts";

export { 解析du, 解析sha256, 读取du, 读取sha256 } from "./read.ts";

export type { 装箱参数 } from "./box.ts";
export { 分装 } from "./box.ts";
