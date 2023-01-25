# `@napi-rs/sysinfo`

![CI](https://github.com/Brooooooklyn/sysinfo/workflows/CI/badge.svg)
[![install size](https://packagephobia.com/badge?p=@napi-rs/sysinfo)](https://packagephobia.com/result?p=@napi-rs/sysinfo)
[![Downloads](https://img.shields.io/npm/dm/@napi-rs/sysinfo.svg?sanitize=true)](https://npmcharts.com/compare/@napi-rs/sysinfo?minimal=true)

> 🚀 Help me to become a full-time open-source developer by [sponsoring me on Github](https://github.com/sponsors/Brooooooklyn)

## `cpuFeatures`

```js
import { cpuFeatures } from '@napi-rs/sysinfo'

console.log(cpuFeatures())
```
<details>
  <summary>
    <code>cpuFeatures()</code> output
  </summary>

```js
{
  arch: 'aarch64',
  brand: 'Apple M1 Max',
  flags: {
    asimd: true,
    pmull: false,
    fp: true,
    fp16: true,
    sve: false,
    crc: true,
    lse: true,
    lse2: false,
    rdm: true,
    rcpc: true,
    rcpc2: true,
    dotprod: true,
    tme: false,
    fhm: true,
    dit: true,
    flagm: true,
    ssbs: true,
    sb: true,
    paca: true,
    pacg: true,
    dpb: true,
    dpb2: true,
    sve2: false,
    sve2Aes: false,
    sve2Sm4: false,
    sve2Sha3: false,
    sve2Bitperm: false,
    frintts: true,
    i8Mm: false,
    f32Mm: false,
    f64Mm: false,
    bf16: false,
    rand: false,
    bti: false,
    mte: false,
    jsconv: true,
    fcma: true,
    aes: true,
    sha2: true,
    sha3: true,
    sm4: false
  }
}
```
</details>

## `sysinfo`

### `CPU info`

```js
import { SysInfo } from '@napi-rs/sysinfo'

const sysinfo = new SysInfo()

for (const cpu of sysinfo.cpus()) {
  console.log(cpu.brand(), cpu.name(), cpu.frequency())
}
// Apple M1 Max cpu0 2427
// Apple M1 Max cpu1 2427
// Apple M1 Max cpu2 3298
// Apple M1 Max cpu3 3298
// Apple M1 Max cpu4 3298
// Apple M1 Max cpu5 3298
// Apple M1 Max cpu6 3298
// Apple M1 Max cpu7 3298
// Apple M1 Max cpu8 3298
// Apple M1 Max cpu9 3298
```
