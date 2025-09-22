use napi_derive::napi;
use std::collections::HashMap;
use sysinfo::{
  DiskUsage as SysDiskUsage, Process, ProcessStatus as SysProcessStatus, Signal as SysSignal,
};

/// Process status enumeration
#[napi]
#[derive(Clone)]
pub enum ProcessStatus {
  /// Process is idle
  Idle,
  /// Process is running
  Run,
  /// Process is sleeping in an interruptible wait
  Sleep,
  /// Process is stopped
  Stop,
  /// Process is a zombie
  Zombie,
  /// Process is being traced
  Tracing,
  /// Process is dead/uninterruptible sleep
  Dead,
  /// Process is wakekill
  Wakekill,
  /// Process is waking
  Waking,
  /// Process is parked
  Parked,
  /// Process is blocked on a lock
  LockBlocked,
  /// Process is waiting in uninterruptible disk sleep
  UninterruptibleDiskSleep,
  /// Process status is unknown
  Unknown,
}

impl From<SysProcessStatus> for ProcessStatus {
  fn from(status: SysProcessStatus) -> Self {
    match status {
      SysProcessStatus::Idle => ProcessStatus::Idle,
      SysProcessStatus::Run => ProcessStatus::Run,
      SysProcessStatus::Sleep => ProcessStatus::Sleep,
      SysProcessStatus::Stop => ProcessStatus::Stop,
      SysProcessStatus::Zombie => ProcessStatus::Zombie,
      SysProcessStatus::Tracing => ProcessStatus::Tracing,
      SysProcessStatus::Dead => ProcessStatus::Dead,
      SysProcessStatus::Wakekill => ProcessStatus::Wakekill,
      SysProcessStatus::Waking => ProcessStatus::Waking,
      SysProcessStatus::Parked => ProcessStatus::Parked,
      SysProcessStatus::LockBlocked => ProcessStatus::LockBlocked,
      SysProcessStatus::UninterruptibleDiskSleep => ProcessStatus::UninterruptibleDiskSleep,
      SysProcessStatus::Unknown(_) => ProcessStatus::Unknown,
    }
  }
}

/// Unix signal enumeration
#[napi]
pub enum Signal {
  /// Hangup detected on controlling terminal
  Hangup,
  /// Interrupt from keyboard
  Interrupt,
  /// Quit from keyboard
  Quit,
  /// Illegal instruction
  Illegal,
  /// Trace/breakpoint trap
  Trap,
  /// Abort signal
  Abort,
  /// IOT trap
  IOT,
  /// Bus error
  Bus,
  /// Floating point exception
  FloatingPointException,
  /// Kill signal
  Kill,
  /// User-defined signal 1
  User1,
  /// Invalid memory reference
  Segv,
  /// User-defined signal 2
  User2,
  /// Broken pipe
  Pipe,
  /// Timer signal
  Alarm,
  /// Termination signal
  Term,
  /// Child stopped or terminated
  Child,
  /// Continue if stopped
  Continue,
  /// Stop process
  Stop,
  /// Stop typed at terminal
  TSTP,
  /// Terminal input for background process
  TTIN,
  /// Terminal output for background process
  TTOU,
  /// Urgent condition on socket
  Urgent,
  /// CPU time limit exceeded
  XCPU,
  /// File size limit exceeded
  XFSZ,
  /// Virtual alarm clock
  VirtualAlarm,
  /// Profiling time expired
  Profiling,
  /// Window resize signal
  Winch,
  /// I/O now possible
  IO,
  /// Pollable event
  Poll,
  /// Power failure
  Power,
  /// Bad argument to routine
  Sys,
}

impl From<Signal> for SysSignal {
  fn from(signal: Signal) -> Self {
    match signal {
      Signal::Hangup => SysSignal::Hangup,
      Signal::Interrupt => SysSignal::Interrupt,
      Signal::Quit => SysSignal::Quit,
      Signal::Illegal => SysSignal::Illegal,
      Signal::Trap => SysSignal::Trap,
      Signal::Abort => SysSignal::Abort,
      Signal::IOT => SysSignal::IOT,
      Signal::Bus => SysSignal::Bus,
      Signal::FloatingPointException => SysSignal::FloatingPointException,
      Signal::Kill => SysSignal::Kill,
      Signal::User1 => SysSignal::User1,
      Signal::Segv => SysSignal::Segv,
      Signal::User2 => SysSignal::User2,
      Signal::Pipe => SysSignal::Pipe,
      Signal::Alarm => SysSignal::Alarm,
      Signal::Term => SysSignal::Term,
      Signal::Child => SysSignal::Child,
      Signal::Continue => SysSignal::Continue,
      Signal::Stop => SysSignal::Stop,
      Signal::TSTP => SysSignal::TSTP,
      Signal::TTIN => SysSignal::TTIN,
      Signal::TTOU => SysSignal::TTOU,
      Signal::Urgent => SysSignal::Urgent,
      Signal::XCPU => SysSignal::XCPU,
      Signal::XFSZ => SysSignal::XFSZ,
      Signal::VirtualAlarm => SysSignal::VirtualAlarm,
      Signal::Profiling => SysSignal::Profiling,
      Signal::Winch => SysSignal::Winch,
      Signal::IO => SysSignal::IO,
      Signal::Poll => SysSignal::Poll,
      Signal::Power => SysSignal::Power,
      Signal::Sys => SysSignal::Sys,
    }
  }
}

/// Disk usage information for a process
#[napi(object)]
#[derive(Clone)]
pub struct DiskUsage {
  /// Number of bytes read from disk
  pub read_bytes: f64,
  /// Number of bytes written to disk
  pub written_bytes: f64,
  /// Total number of bytes read
  pub total_read_bytes: f64,
  /// Total number of bytes written
  pub total_written_bytes: f64,
}

impl From<SysDiskUsage> for DiskUsage {
  fn from(usage: SysDiskUsage) -> Self {
    Self {
      read_bytes: usage.read_bytes as f64,
      written_bytes: usage.written_bytes as f64,
      total_read_bytes: usage.total_read_bytes as f64,
      total_written_bytes: usage.total_written_bytes as f64,
    }
  }
}

/// A process running on the system
#[napi]
pub struct ProcessWrapper {
  pub(crate) pid: u32,
  pub(crate) parent: Option<u32>,
  pub(crate) name: String,
  pub(crate) cmd: Vec<String>,
  pub(crate) exe: Option<String>,
  pub(crate) cwd: Option<String>,
  pub(crate) root: Option<String>,
  pub(crate) memory: u64,
  pub(crate) virtual_memory: u64,
  pub(crate) cpu_usage: f32,
  pub(crate) disk_usage: DiskUsage,
  pub(crate) status: ProcessStatus,
  pub(crate) start_time: u64,
  pub(crate) run_time: u64,
  pub(crate) user_id: Option<String>,
  pub(crate) effective_user_id: Option<String>,
  pub(crate) group_id: Option<String>,
  pub(crate) effective_group_id: Option<String>,
  pub(crate) session_id: Option<u32>,
  pub(crate) environ: HashMap<String, String>,
}

#[napi]
impl ProcessWrapper {
  /// Returns the process identifier (PID)
  #[napi]
  pub fn pid(&self) -> u32 {
    self.pid
  }

  /// Returns the parent process identifier (PPID)
  #[napi]
  pub fn parent(&self) -> Option<u32> {
    self.parent
  }

  /// Returns the process name
  #[napi]
  pub fn name(&self) -> String {
    self.name.clone()
  }

  /// Returns the command line arguments
  #[napi]
  pub fn cmd(&self) -> Vec<String> {
    self.cmd.clone()
  }

  /// Returns the process executable path
  #[napi]
  pub fn exe(&self) -> Option<String> {
    self.exe.clone()
  }

  /// Returns the process current working directory
  #[napi]
  pub fn cwd(&self) -> Option<String> {
    self.cwd.clone()
  }

  /// Returns the process root directory
  #[napi]
  pub fn root(&self) -> Option<String> {
    self.root.clone()
  }

  /// Returns the amount of memory currently used by the process in bytes
  #[napi]
  pub fn memory(&self) -> u64 {
    self.memory
  }

  /// Returns the amount of virtual memory used by the process in bytes
  #[napi]
  pub fn virtual_memory(&self) -> u64 {
    self.virtual_memory
  }

  /// Returns the CPU usage percentage (0.0 to 100.0)
  #[napi]
  pub fn cpu_usage(&self) -> f32 {
    self.cpu_usage
  }

  /// Returns the disk usage information
  #[napi]
  pub fn disk_usage(&self) -> DiskUsage {
    self.disk_usage.clone()
  }

  /// Returns the process status
  #[napi]
  pub fn status(&self) -> ProcessStatus {
    self.status.clone()
  }

  /// Returns the process start time (seconds since epoch)
  #[napi]
  pub fn start_time(&self) -> u64 {
    self.start_time
  }

  /// Returns the process run time in seconds
  #[napi]
  pub fn run_time(&self) -> u64 {
    self.run_time
  }

  /// Returns the user ID of the process owner
  #[napi]
  pub fn user_id(&self) -> Option<String> {
    self.user_id.clone()
  }

  /// Returns the effective user ID of the process
  #[napi]
  pub fn effective_user_id(&self) -> Option<String> {
    self.effective_user_id.clone()
  }

  /// Returns the group ID of the process
  #[napi]
  pub fn group_id(&self) -> Option<String> {
    self.group_id.clone()
  }

  /// Returns the effective group ID of the process
  #[napi]
  pub fn effective_group_id(&self) -> Option<String> {
    self.effective_group_id.clone()
  }

  /// Returns the session ID of the process
  #[napi]
  pub fn session_id(&self) -> Option<u32> {
    self.session_id
  }

  /// Returns the environment variables of the process
  #[napi]
  pub fn environ(&self) -> HashMap<String, String> {
    self.environ.clone()
  }
}

impl From<&Process> for ProcessWrapper {
  fn from(process: &Process) -> Self {
    let environ = process
      .environ()
      .iter()
      .map(|var| {
        let var_str = var.to_string_lossy();
        if let Some((key, value)) = var_str.split_once('=') {
          (key.to_string(), value.to_string())
        } else {
          (var_str.to_string(), String::new())
        }
      })
      .collect();

    Self {
      pid: process.pid().as_u32(),
      parent: process.parent().map(|pid| pid.as_u32()),
      name: process.name().to_string_lossy().to_string(),
      cmd: process
        .cmd()
        .iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect(),
      exe: process.exe().map(|path| path.to_string_lossy().to_string()),
      cwd: process.cwd().map(|path| path.to_string_lossy().to_string()),
      root: process
        .root()
        .map(|path| path.to_string_lossy().to_string()),
      memory: process.memory(),
      virtual_memory: process.virtual_memory(),
      cpu_usage: process.cpu_usage(),
      disk_usage: process.disk_usage().into(),
      status: process.status().into(),
      start_time: process.start_time(),
      run_time: process.run_time(),
      user_id: process.user_id().map(|uid| uid.to_string()),
      effective_user_id: process.effective_user_id().map(|uid| uid.to_string()),
      group_id: process.group_id().map(|gid| gid.to_string()),
      effective_group_id: process.effective_group_id().map(|gid| gid.to_string()),
      session_id: process.session_id().map(|sid| sid.as_u32()),
      environ,
    }
  }
}
