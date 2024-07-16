/**
 * 数据大小格式化显示: KB, MB, GB, TB
 */

/**
 * K, M, G, T 进制: 1024.
 */
const K = 1024;

// 具体的单位
const U_B = "B";
const U_K = "KB";
const U_M = "MB";
const U_G = "GB";
const U_T = "TB";
// 单位的系数
const K_B = 1;
const K_K = K;
const K_M = K * K;
const K_G = K * K * K;
const K_T = K * K * K * K;

/**
 * 字节转换带单位大小.
 */
export function 显示大小(字节: number): string {
  if (K_K > 字节) {
    // Byte
    return 字节 + U_B;
  } else if (K_M > 字节) {
    // KB
    return (字节 / K_K).toFixed(1) + U_K;
  } else if (K_G > 字节) {
    // MB
    return (字节 / K_M).toFixed(1) + U_M;
  } else if (K_T > 字节) {
    // GB
    return (字节 / K_G).toFixed(1) + U_G;
  } else {
    // TB
    return (字节 / K_T).toFixed(1) + U_T;
  }
}

/**
 * 带单位大小转换成字节.
 */
export function 解析大小(大小: string): number {
  // 系数
  let k = K_B;
  // 单位
  let u = "";

  // 检查单位
  if (大小.endsWith(U_B)) {
    u = U_B;
  } else if (大小.endsWith(U_K)) {
    u = U_K;
    k = K_K;
  } else if (大小.endsWith(U_M)) {
    u = U_M;
    k = K_M;
  } else if (大小.endsWith(U_G)) {
    u = U_G;
    k = K_G;
  } else if (大小.endsWith(U_T)) {
    u = U_T;
    k = K_T;
  }

  // 去除单位
  const 数字 = 大小.slice(大小.length - u.length, 大小.length);
  const n = Number.parseFloat(数字);
  if (Number.isNaN(n) || (!Number.isFinite(n))) {
    throw new Error("bad size: " + 大小);
  }

  return n * k;
}
