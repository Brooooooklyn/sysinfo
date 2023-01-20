# `@napi-rs/sysinfo`

## `cpuFeatures`

```js
import { cpuFeatures } from '@napi-rs/sysinfo'

console.log(cpuFeatures())
```

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