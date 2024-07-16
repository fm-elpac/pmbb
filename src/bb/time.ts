/**
 * 时间相关.
 */

import { format } from "@std/datetime";

/**
 * 生成时间 (项目) 标记 (UTC).
 *
 * 格式: 20240716_220837
 */
export function 时间标记(d: Date): string {
  return format(d, "yyyyMMdd_HHmmss");
}

// TODO
