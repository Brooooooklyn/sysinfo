# `@napi-rs/sysinfo`

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
// Apple M1 Max cpu0 2427256522240n
// Apple M1 Max cpu1 2427256522240n
// Apple M1 Max cpu2 3299134883328n
// Apple M1 Max cpu3 3299134883328n
// Apple M1 Max cpu4 3299134883328n
// Apple M1 Max cpu5 3299134883328n
// Apple M1 Max cpu6 3299134883328n
// Apple M1 Max cpu7 3299134883328n
// Apple M1 Max cpu8 3299134883328n
// Apple M1 Max cpu9 3299134883328n
```
