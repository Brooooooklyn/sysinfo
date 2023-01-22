/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface CpuFeatures {
  arch: string
  brand?: string
  family?: number
  model?: number
  steppingId?: number
  flags: CpuFeaturesFlags
}
export interface CpuFeaturesFlags {
  fpu: boolean
  aes: boolean
  pclmulqdq: boolean
  rdrand: boolean
  rdseed: boolean
  tsc: boolean
  mmx: boolean
  sse: boolean
  sse2: boolean
  sse3: boolean
  ssse3: boolean
  sse41: boolean
  sse42: boolean
  sse4A: boolean
  sha: boolean
  avx: boolean
  avx2: boolean
  avx512F: boolean
  avx512Cd: boolean
  avx512Er: boolean
  avx512Pf: boolean
  avx512Bw: boolean
  avx512Dq: boolean
  avx512Vl: boolean
  avx512Ifma: boolean
  avx512Vbmi: boolean
  avx512Vpopcntdq: boolean
  avx512Vbmi2: boolean
  avx512Gfni: boolean
  avx512Vaes: boolean
  avx512Vpclmulqdq: boolean
  avx512Vnni: boolean
  avx512Bitalg: boolean
  avx512Bf16: boolean
  avx512Vp2Intersect: boolean
  f16C: boolean
  fma: boolean
  bmi1: boolean
  bmi2: boolean
  abm: boolean
  lzcnt: boolean
  tbm: boolean
  popcnt: boolean
  fxsr: boolean
  xsave: boolean
  xsaveopt: boolean
  xsaves: boolean
  xsavec: boolean
  cmpxchg16B: boolean
  adx: boolean
  rtm: boolean
}
export function cpuFeatures(): CpuFeatures
/**
 * A Object representing system load average value.
 *
 * ```js
 * import { SysInfo } from '@napi-rs/sysinfo';
 * const s = new SysInfo();
 * const loadAvg = s.loadAverage();
 *
 * console.log(
 *   `one minute: ${loadAvg.one}%, five minutes: ${loadAvg.five}%, fifteen minutes: ${loadAvg.fifteen}%`,
 * )
 * ```
 */
export interface LoadAvg {
  /** Average load within one minute. */
  one: number
  /** Average load within five minutes. */
  five: number
  /** Average load within fifteen minutes. */
  fifteen: number
}
export class Cpu {
  usage(): number
  name(): string
  name(): string
  /** Cpu frequency in `MHz` */
  frequency(): number
  vendorId(): string
  brand(): string
}
export class SysInfo {
  constructor()
  globalCpuInfo(): Cpu
  cpus(): Array<Cpu>
  refreshMemory(): void
  totalMemory(): bigint
  freeMemory(): bigint
  availableMemory(): bigint
  usedMemory(): bigint
  totalSwap(): bigint
  freeSwap(): bigint
  usedSwap(): bigint
  uptime(): void
  bootTime(): bigint
  systemName(): string | null
  longOsVersion(): string | null
  hostName(): string | null
  kernelVersion(): string | null
  osVersion(): string | null
  distribution(): string
  loadAverage(): LoadAvg
  refreshComponentsList(): void
}
