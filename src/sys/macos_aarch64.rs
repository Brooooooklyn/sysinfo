#![allow(non_camel_case_types)]

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
use core_foundation::dictionary::{CFDictionaryRef, CFMutableDictionaryRef};

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
pub type io_object_t = libc::mach_port_t;
pub type io_iterator_t = io_object_t;
pub type io_registry_entry_t = io_object_t;

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
extern "C" {
  pub static kIOMasterPortDefault: libc::mach_port_t;

  pub fn IOMasterPort(
    bootstrapPort: libc::mach_port_t,
    masterPort: *mut libc::mach_port_t,
  ) -> libc::kern_return_t;
  pub fn IOServiceMatching(name: *const libc::c_char) -> CFMutableDictionaryRef;

  pub fn IOServiceGetMatchingServices(
    masterPort: libc::mach_port_t,
    matching: CFDictionaryRef,
    existing: *mut io_object_t,
  ) -> libc::kern_return_t;

  pub fn IOObjectRelease(object: io_object_t) -> libc::kern_return_t;

  pub fn IOIteratorNext(iterator: io_iterator_t) -> io_object_t;

  pub fn IORegistryEntryCreateCFProperties(
    entry: io_registry_entry_t,
    property_name: *mut core_foundation::dictionary::CFMutableDictionaryRef,
    buffer: *const libc::c_void,
    size: u32,
  ) -> libc::kern_return_t;

}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernStatus {
  KernSuccess = 0,
  KernInvalidAddress = 1,
  KernProtectionFailure = 2,
  KernNoSpace = 3,
  KernInvalidArgument = 4,
  KernFailure = 5,
  KernResourceShortage = 6,
  KernNotReceiver = 7,
  KernNoAccess = 8,
  KernMemoryFailure = 9,
  KernMemoryError = 10,
  KernAlreadyInSet = 11,
  KernNotInSet = 12,
  KernNameExists = 13,
  KernAborted = 14,
  KernInvalidName = 15,
  KernInvalidTask = 16,
  KernInvalidRight = 17,
  KernInvalidValue = 18,
  KernUrefsOverflow = 19,
  KernInvalidCapability = 20,
  KernRightExists = 21,
  KernInvalidHost = 22,
  KernMemoryPresent = 23,
  KernMemoryDataMoved = 24,
  KernMemoryRestartCopy = 25,
  KernInvalidProcessorSet = 26,
  KernPolicyLimit = 27,
  KernInvalidPolicy = 28,
  KernInvalidObject = 29,
  KernAlreadyWaiting = 30,
  KernDefaultSet = 31,
  KernExceptionProtected = 32,
  KernInvalidLedger = 33,
  KernInvalidMemoryControl = 34,
  KernInvalidSecurity = 35,
  KernNotDepressed = 36,
  KernTerminated = 37,
  KernLockSetDestroyed = 38,
  KernLockUnstable = 39,
  KernLockOwned = 40,
  KernLockOwnedSelf = 41,
  KernSemaphoreDestroyed = 42,
  KernRpcServerTerminated = 43,
  KernRpcTerminateOrphan = 44,
  KernRpcContinueOrphan = 45,
  KernNotSupported = 46,
  KernNodeDown = 47,
  KernNotWaiting = 48,
  KernOperationTimedOut = 49,
  KernCodesignError = 50,
  KernPolicyStatic = 51,
  KernInsufficientBufferSize = 52,
  KernUnknown = 4096,
}

impl From<libc::c_int> for KernStatus {
  fn from(value: libc::c_int) -> Self {
    match value {
      0 => KernStatus::KernSuccess,
      1 => KernStatus::KernInvalidAddress,
      2 => KernStatus::KernProtectionFailure,
      3 => KernStatus::KernNoSpace,
      4 => KernStatus::KernInvalidArgument,
      5 => KernStatus::KernFailure,
      6 => KernStatus::KernResourceShortage,
      7 => KernStatus::KernNotReceiver,
      8 => KernStatus::KernNoAccess,
      9 => KernStatus::KernMemoryFailure,
      10 => KernStatus::KernMemoryError,
      11 => KernStatus::KernAlreadyInSet,
      12 => KernStatus::KernNotInSet,
      13 => KernStatus::KernNameExists,
      14 => KernStatus::KernAborted,
      15 => KernStatus::KernInvalidName,
      16 => KernStatus::KernInvalidTask,
      17 => KernStatus::KernInvalidRight,
      18 => KernStatus::KernInvalidValue,
      19 => KernStatus::KernUrefsOverflow,
      20 => KernStatus::KernInvalidCapability,
      21 => KernStatus::KernRightExists,
      22 => KernStatus::KernInvalidHost,
      23 => KernStatus::KernMemoryPresent,
      24 => KernStatus::KernMemoryDataMoved,
      25 => KernStatus::KernMemoryRestartCopy,
      26 => KernStatus::KernInvalidProcessorSet,
      27 => KernStatus::KernPolicyLimit,
      28 => KernStatus::KernInvalidPolicy,
      29 => KernStatus::KernInvalidObject,
      30 => KernStatus::KernAlreadyWaiting,
      31 => KernStatus::KernDefaultSet,
      32 => KernStatus::KernExceptionProtected,
      33 => KernStatus::KernInvalidLedger,
      34 => KernStatus::KernInvalidMemoryControl,
      35 => KernStatus::KernInvalidSecurity,
      36 => KernStatus::KernNotDepressed,
      37 => KernStatus::KernTerminated,
      38 => KernStatus::KernLockSetDestroyed,
      39 => KernStatus::KernLockUnstable,
      40 => KernStatus::KernLockOwned,
      41 => KernStatus::KernLockOwnedSelf,
      42 => KernStatus::KernSemaphoreDestroyed,
      43 => KernStatus::KernRpcServerTerminated,
      44 => KernStatus::KernRpcTerminateOrphan,
      45 => KernStatus::KernRpcContinueOrphan,
      46 => KernStatus::KernNotSupported,
      47 => KernStatus::KernNodeDown,
      48 => KernStatus::KernNotWaiting,
      49 => KernStatus::KernOperationTimedOut,
      50 => KernStatus::KernCodesignError,
      51 => KernStatus::KernPolicyStatic,
      52 => KernStatus::KernInsufficientBufferSize,
      _ => KernStatus::KernUnknown,
    }
  }
}
