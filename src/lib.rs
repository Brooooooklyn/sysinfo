#![deny(clippy::all)]

use cpu::Cpu;
use process::ProcessWrapper;
use napi::{bindgen_prelude::Reference, Env, Result};

use napi_derive::napi;

mod cpu;
mod process;
mod sys;

#[napi(object)]
/// Process refresh configuration for controlling what information to update
pub struct ProcessRefreshConfig {
  /// Refresh memory information (default: true)
  pub memory: Option<bool>,
  /// Refresh CPU usage information (default: true)
  pub cpu: Option<bool>,
  /// Refresh disk usage information (default: true)
  pub disk_usage: Option<bool>,
  /// Refresh executable path information (default: true)
  pub exe: Option<bool>,
  /// Refresh command line arguments (default: false)
  pub cmd: Option<bool>,
  /// Refresh environment variables (default: false)
  pub environ: Option<bool>,
  /// Refresh current working directory (default: false)
  pub cwd: Option<bool>,
  /// Refresh user information (default: false)
  pub user: Option<bool>,
  /// Refresh tasks/threads information (default: false)
  pub tasks: Option<bool>,
}

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

  use sysinfo::{CpuRefreshKind, RefreshKind};

  let sysinfo = sysinfo::System::new_with_specifics(
    RefreshKind::everything().with_cpu(CpuRefreshKind::everything()),
  );
  let cpu = &sysinfo.cpus()[0];
  CpuFeatures {
    arch: std::env::consts::ARCH.to_string(),
    brand: Some(cpu.brand().to_string()),
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
      avx512gfni: is_x86_feature_detected!("gfni"),
      avx512vaes: is_x86_feature_detected!("vaes"),
      avx512vpclmulqdq: is_x86_feature_detected!("vpclmulqdq"),
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

#[napi(object)]
/// A Object representing system load average value.
///
/// ```js
/// import { SysInfo } from '@napi-rs/sysinfo';
/// const s = new SysInfo();
/// const loadAvg = s.loadAverage();
///
/// console.log(
///   `one minute: ${loadAvg.one}%, five minutes: ${loadAvg.five}%, fifteen minutes: ${loadAvg.fifteen}%`,
/// )
/// ```
pub struct LoadAvg {
  /// Average load within one minute.
  pub one: f64,
  /// Average load within five minutes.
  pub five: f64,
  /// Average load within fifteen minutes.
  pub fifteen: f64,
}

impl From<sysinfo::LoadAvg> for LoadAvg {
  fn from(value: sysinfo::LoadAvg) -> Self {
    Self {
      one: value.one,
      five: value.five,
      fifteen: value.fifteen,
    }
  }
}

#[napi]
pub struct SysInfo {
  system: sysinfo::System,
}

#[napi]
impl SysInfo {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      system: sysinfo::System::new_with_specifics(sysinfo::RefreshKind::everything()),
    }
  }

  #[napi]
  pub fn cpus(&self, env: Env, this: Reference<SysInfo>) -> Result<Vec<Cpu>> {
    let cpus = this.share_with(env, |sys| Ok(sys.system.cpus()))?;
    Ok(cpus.iter().map(|inner| Cpu { inner }).collect())
  }

  #[napi]
  pub fn refresh_memory(&mut self) {
    self.system.refresh_memory();
  }

  #[napi]
  pub fn total_memory(&self) -> u64 {
    self.system.total_memory()
  }

  #[napi]
  pub fn free_memory(&self) -> u64 {
    self.system.free_memory()
  }

  #[napi]
  pub fn available_memory(&self) -> u64 {
    self.system.available_memory()
  }

  #[napi]
  pub fn used_memory(&self) -> u64 {
    self.system.used_memory()
  }

  #[napi]
  pub fn total_swap(&self) -> u64 {
    self.system.total_swap()
  }

  #[napi]
  pub fn free_swap(&self) -> u64 {
    self.system.free_swap()
  }

  #[napi]
  pub fn used_swap(&self) -> u64 {
    self.system.used_swap()
  }

  #[napi]
  pub fn uptime(&self) {
    sysinfo::System::uptime();
  }

  #[napi]
  pub fn boot_time(&self) -> u64 {
    sysinfo::System::boot_time()
  }

  #[napi]
  pub fn system_name(&self) -> Option<String> {
    sysinfo::System::name()
  }

  #[napi]
  pub fn long_os_version(&self) -> Option<String> {
    sysinfo::System::long_os_version()
  }

  #[napi]
  pub fn host_name(&self) -> Option<String> {
    sysinfo::System::host_name()
  }

  #[napi]
  pub fn kernel_version(&self) -> Option<String> {
    sysinfo::System::kernel_version()
  }

  #[napi]
  pub fn os_version(&self) -> Option<String> {
    sysinfo::System::os_version()
  }

  #[napi]
  pub fn distribution(&self) -> String {
    sysinfo::System::distribution_id()
  }

  #[napi]
  pub fn load_average(&self) -> LoadAvg {
    sysinfo::System::load_average().into()
  }

  #[napi]
  pub fn refresh_components_list(&mut self) {
    self.system.refresh_all();
  }

  /// Refresh processes information with detailed control
  #[napi]
  pub fn refresh_processes(&mut self, remove_dead: Option<bool>) -> u32 {
    let remove_dead_processes = remove_dead.unwrap_or(true);
    self.system.refresh_processes(sysinfo::ProcessesToUpdate::All, remove_dead_processes) as u32
  }

  /// Refresh specific processes by PIDs with detailed control
  #[napi]
  pub fn refresh_processes_specifics(&mut self, pids: Vec<u32>, remove_dead: Option<bool>, refresh_config: Option<ProcessRefreshConfig>) -> u32 {
    let pids: Vec<sysinfo::Pid> = pids.into_iter().map(|pid| sysinfo::Pid::from(pid as usize)).collect();
    let remove_dead_processes = remove_dead.unwrap_or(true);
    
    // ?
    let refresh_kind = if let Some(config) = refresh_config {
      let mut kind = sysinfo::ProcessRefreshKind::nothing();
      
      if config.memory.unwrap_or(true) {
        kind = kind.with_memory();
      }
      if config.cpu.unwrap_or(true) {
        kind = kind.with_cpu();
      }
      if config.disk_usage.unwrap_or(true) {
        kind = kind.with_disk_usage();
      }
      if config.exe.unwrap_or(true) {
        kind = kind.with_exe(sysinfo::UpdateKind::OnlyIfNotSet);
      }
      if config.cmd.unwrap_or(false) {
        kind = kind.with_cmd(sysinfo::UpdateKind::OnlyIfNotSet);
      }
      if config.environ.unwrap_or(false) {
        kind = kind.with_environ(sysinfo::UpdateKind::OnlyIfNotSet);
      }
      if config.cwd.unwrap_or(false) {
        kind = kind.with_cwd(sysinfo::UpdateKind::OnlyIfNotSet);
      }
      if config.user.unwrap_or(false) {
        kind = kind.with_user(sysinfo::UpdateKind::OnlyIfNotSet);
      }
      if config.tasks.unwrap_or(false) {
        kind = kind.with_tasks();
      }
      
      kind
    } else {
      sysinfo::ProcessRefreshKind::nothing()
        .with_memory()
        .with_cpu()
        .with_disk_usage()
        .with_exe(sysinfo::UpdateKind::OnlyIfNotSet)
    };
    
    self.system.refresh_processes_specifics(
      sysinfo::ProcessesToUpdate::Some(&pids),
      remove_dead_processes,
      refresh_kind
    ) as u32
  }

  /// Get all processes
  #[napi]
  pub fn processes(&self) -> Vec<ProcessWrapper> {
    self.system
      .processes()
      .values()
      .map(|process| ProcessWrapper::from(process))
      .collect()
  }

  /// Get process by PID
  #[napi]
  pub fn process_by_pid(&self, pid: u32) -> Option<ProcessWrapper> {
    self.system
      .process(sysinfo::Pid::from(pid as usize))
      .map(|process| ProcessWrapper::from(process))
  }

  /// Get processes by name
  #[napi]
  pub fn processes_by_name(&self, name: String) -> Vec<ProcessWrapper> {
    self.system
      .processes_by_name(name.as_ref())
      .map(|process| ProcessWrapper::from(process))
      .collect()
  }

  /// Get processes by exact name
  #[napi]
  pub fn processes_by_exact_name(&self, name: String) -> Vec<ProcessWrapper> {
    self.system
      .processes_by_exact_name(name.as_ref())
      .map(|process| ProcessWrapper::from(process))
      .collect()
  }
}
