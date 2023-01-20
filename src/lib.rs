#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct CpuFeatures {
  pub arch: String,
  pub brand: Option<String>,
  pub family: Option<u32>,
  pub model: Option<u32>,
  pub stepping_id: Option<u32>,
  pub flags: CpuFeaturesFlags,
}

#[napi(object)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub struct CpuFeaturesFlags {
  pub fpu: bool,
  pub aes: bool,
  pub pclmulqdq: bool,
  pub rdrand: bool,
  pub rdseed: bool,
  pub tsc: bool,
  pub mmx: bool,
  pub sse: bool,
  pub sse2: bool,
  pub sse3: bool,
  pub ssse3: bool,
  pub sse4_1: bool,
  pub sse4_2: bool,
  pub sse4a: bool,
  pub sha: bool,
  pub avx: bool,
  pub avx2: bool,
  pub avx512f: bool,
  pub avx512cd: bool,
  pub avx512er: bool,
  pub avx512pf: bool,
  pub avx512bw: bool,
  pub avx512dq: bool,
  pub avx512vl: bool,
  pub avx512ifma: bool,
  pub avx512vbmi: bool,
  pub avx512vpopcntdq: bool,
  pub avx512vbmi2: bool,
  pub avx512gfni: bool,
  pub avx512vaes: bool,
  pub avx512vpclmulqdq: bool,
  pub avx512vnni: bool,
  pub avx512bitalg: bool,
  pub avx512bf16: bool,
  pub avx512vp2intersect: bool,
  pub f16c: bool,
  pub fma: bool,
  pub bmi1: bool,
  pub bmi2: bool,
  pub abm: bool,
  pub lzcnt: bool,
  pub tbm: bool,
  pub popcnt: bool,
  pub fxsr: bool,
  pub xsave: bool,
  pub xsaveopt: bool,
  pub xsaves: bool,
  pub xsavec: bool,
  pub cmpxchg16b: bool,
  pub adx: bool,
  pub rtm: bool,
}

#[cfg(target_arch = "arm")]
#[napi(object)]
pub struct CpuFeaturesFlags {
  pub neon: bool,
  pub pmull: bool,
  pub crc: bool,
  pub crypto: bool,
  pub aes: bool,
  pub sha2: bool,
  pub i8mm: bool,
  pub v7: bool,
  pub vfp2: bool,
  pub vfp3: bool,
  pub vfp4: bool,
}

#[napi(object)]
#[cfg(target_arch = "aarch64")]
pub struct CpuFeaturesFlags {
  pub asimd: bool,
  pub pmull: bool,
  pub fp: bool,
  pub fp16: bool,
  pub sve: bool,
  pub crc: bool,
  pub lse: bool,
  pub lse2: bool,
  pub rdm: bool,
  pub rcpc: bool,
  pub rcpc2: bool,
  pub dotprod: bool,
  pub tme: bool,
  pub fhm: bool,
  pub dit: bool,
  pub flagm: bool,
  pub ssbs: bool,
  pub sb: bool,
  pub paca: bool,
  pub pacg: bool,
  pub dpb: bool,
  pub dpb2: bool,
  pub sve2: bool,
  pub sve2_aes: bool,
  pub sve2_sm4: bool,
  pub sve2_sha3: bool,
  pub sve2_bitperm: bool,
  pub frintts: bool,
  pub i8mm: bool,
  pub f32mm: bool,
  pub f64mm: bool,
  pub bf16: bool,
  pub rand: bool,
  pub bti: bool,
  pub mte: bool,
  pub jsconv: bool,
  pub fcma: bool,
  pub aes: bool,
  pub sha2: bool,
  pub sha3: bool,
  pub sm4: bool,
}

#[napi]
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn cpu_features() -> CpuFeatures {
  #[cfg(target_arch = "aarch64")]
  use std::arch::is_aarch64_feature_detected;

  use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, SystemExt};

  let sysinfo =
    sysinfo::System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::new()));
  let cpu = sysinfo.cpus().get(0);
  CpuFeatures {
    arch: std::env::consts::ARCH.to_string(),
    brand: cpu.map(|cpu| cpu.brand().to_string()),
    family: None,
    model: None,
    stepping_id: None,
    #[cfg(target_arch = "aarch64")]
    flags: CpuFeaturesFlags {
      asimd: is_aarch64_feature_detected!("asimd"),
      pmull: is_aarch64_feature_detected!("pmull"),
      fp: is_aarch64_feature_detected!("fp"),
      fp16: is_aarch64_feature_detected!("fp16"),
      sve: is_aarch64_feature_detected!("sve"),
      crc: is_aarch64_feature_detected!("crc"),
      lse: is_aarch64_feature_detected!("lse"),
      lse2: is_aarch64_feature_detected!("lse2"),
      rdm: is_aarch64_feature_detected!("rdm"),
      rcpc: is_aarch64_feature_detected!("rcpc"),
      rcpc2: is_aarch64_feature_detected!("rcpc2"),
      dotprod: is_aarch64_feature_detected!("dotprod"),
      tme: is_aarch64_feature_detected!("tme"),
      fhm: is_aarch64_feature_detected!("fhm"),
      dit: is_aarch64_feature_detected!("dit"),
      flagm: is_aarch64_feature_detected!("flagm"),
      ssbs: is_aarch64_feature_detected!("ssbs"),
      sb: is_aarch64_feature_detected!("sb"),
      paca: is_aarch64_feature_detected!("paca"),
      pacg: is_aarch64_feature_detected!("pacg"),
      dpb: is_aarch64_feature_detected!("dpb"),
      dpb2: is_aarch64_feature_detected!("dpb2"),
      sve2: is_aarch64_feature_detected!("sve2"),
      sve2_aes: is_aarch64_feature_detected!("sve2-aes"),
      sve2_sm4: is_aarch64_feature_detected!("sve2-sm4"),
      sve2_sha3: is_aarch64_feature_detected!("sve2-sha3"),
      sve2_bitperm: is_aarch64_feature_detected!("sve2-bitperm"),
      frintts: is_aarch64_feature_detected!("frintts"),
      i8mm: is_aarch64_feature_detected!("i8mm"),
      f32mm: is_aarch64_feature_detected!("f32mm"),
      f64mm: is_aarch64_feature_detected!("f64mm"),
      bf16: is_aarch64_feature_detected!("bf16"),
      rand: is_aarch64_feature_detected!("rand"),
      bti: is_aarch64_feature_detected!("bti"),
      mte: is_aarch64_feature_detected!("mte"),
      jsconv: is_aarch64_feature_detected!("jsconv"),
      fcma: is_aarch64_feature_detected!("fcma"),
      aes: is_aarch64_feature_detected!("aes"),
      sha2: is_aarch64_feature_detected!("sha2"),
      sha3: is_aarch64_feature_detected!("sha3"),
      sm4: is_aarch64_feature_detected!("sm4"),
    },
    #[cfg(target_arch = "arm")]
    flags: CpuFeaturesFlags {
      neon: false,
      pmull: false,
      crc: false,
      crypto: false,
      aes: false,
      sha2: false,
      i8mm: false,
      v7: false,
      vfp2: false,
      vfp3: false,
      vfp4: false,
    },
  }
}

#[napi]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn cpu_features() -> CpuFeatures {
  use std::arch::is_x86_feature_detected;

  use raw_cpuid::CpuId;

  let cpuid = CpuId::new();
  let cpu_feature_info = cpuid.get_feature_info();
  let cpu_feature_info = cpu_feature_info.as_ref();

  CpuFeatures {
    arch: std::env::consts::ARCH.to_string(),
    brand: cpuid
      .get_processor_brand_string()
      .map(|brand| brand.as_str().to_string()),
    family: cpu_feature_info.map(|info| info.family_id() as u32),
    model: cpu_feature_info.map(|info| info.model_id() as u32),
    stepping_id: cpu_feature_info.map(|info| info.stepping_id() as u32),
    flags: CpuFeaturesFlags {
      fpu: cpu_feature_info.map(|info| info.has_fpu()).unwrap_or(false),
      aes: is_x86_feature_detected!("aes"),
      pclmulqdq: is_x86_feature_detected!("pclmulqdq"),
      rdrand: is_x86_feature_detected!("rdrand"),
      rdseed: is_x86_feature_detected!("rdseed"),
      tsc: is_x86_feature_detected!("tsc"),
      mmx: is_x86_feature_detected!("mmx"),
      sse: is_x86_feature_detected!("sse"),
      sse2: is_x86_feature_detected!("sse2"),
      sse3: is_x86_feature_detected!("sse3"),
      ssse3: is_x86_feature_detected!("ssse3"),
      sse4_1: is_x86_feature_detected!("sse4.1"),
      sse4_2: is_x86_feature_detected!("sse4.2"),
      sse4a: is_x86_feature_detected!("sse4a"),
      sha: is_x86_feature_detected!("sha"),
      avx: is_x86_feature_detected!("avx"),
      avx2: is_x86_feature_detected!("avx2"),
      avx512f: is_x86_feature_detected!("avx512f"),
      avx512cd: is_x86_feature_detected!("avx512cd"),
      avx512er: is_x86_feature_detected!("avx512er"),
      avx512pf: is_x86_feature_detected!("avx512pf"),
      avx512bw: is_x86_feature_detected!("avx512bw"),
      avx512dq: is_x86_feature_detected!("avx512dq"),
      avx512vl: is_x86_feature_detected!("avx512vl"),
      avx512ifma: is_x86_feature_detected!("avx512ifma"),
      avx512vbmi: is_x86_feature_detected!("avx512vbmi"),
      avx512vpopcntdq: is_x86_feature_detected!("avx512vpopcntdq"),
      avx512vbmi2: is_x86_feature_detected!("avx512vbmi2"),
      avx512gfni: is_x86_feature_detected!("avx512gfni"),
      avx512vaes: is_x86_feature_detected!("avx512vaes"),
      avx512vpclmulqdq: is_x86_feature_detected!("avx512vpclmulqdq"),
      avx512vnni: is_x86_feature_detected!("avx512vnni"),
      avx512bitalg: is_x86_feature_detected!("avx512bitalg"),
      avx512bf16: is_x86_feature_detected!("avx512bf16"),
      avx512vp2intersect: is_x86_feature_detected!("avx512vp2intersect"),
      f16c: is_x86_feature_detected!("f16c"),
      fma: is_x86_feature_detected!("fma"),
      bmi1: is_x86_feature_detected!("bmi1"),
      bmi2: is_x86_feature_detected!("bmi2"),
      abm: is_x86_feature_detected!("abm"),
      lzcnt: is_x86_feature_detected!("lzcnt"),
      tbm: is_x86_feature_detected!("tbm"),
      popcnt: is_x86_feature_detected!("popcnt"),
      fxsr: is_x86_feature_detected!("fxsr"),
      xsave: is_x86_feature_detected!("xsave"),
      xsaveopt: is_x86_feature_detected!("xsaveopt"),
      xsaves: is_x86_feature_detected!("xsaves"),
      xsavec: is_x86_feature_detected!("xsavec"),
      cmpxchg16b: is_x86_feature_detected!("cmpxchg16b"),
      adx: is_x86_feature_detected!("adx"),
      rtm: is_x86_feature_detected!("rtm"),
    },
  }
}
