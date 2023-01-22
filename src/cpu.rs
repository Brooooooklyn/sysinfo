use napi_derive::napi;
#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
use once_cell::sync::Lazy;
use sysinfo::CpuExt;

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
/// (efficiency, performance)
static CORE_FREQUENCY: Lazy<napi::Result<(u64, u64)>> = Lazy::new(|| unsafe {
  use core_foundation::{
    base::{kCFAllocatorDefault, CFType, TCFType},
    data::{CFData, CFDataGetBytePtr},
    dictionary::{CFDictionary, CFMutableDictionary, CFMutableDictionaryRef},
    string::CFString,
  };
  use libc::{mach_port_t, KERN_SUCCESS, MACH_PORT_NULL};

  use crate::sys::macos_aarch64::*;

  let mut master_port: mach_port_t = MACH_PORT_NULL as _;
  let mut efficiency_core_frequency = 0;
  let mut performance_core_frequency = 0;
  let kr = IOMasterPort(kIOMasterPortDefault, &mut master_port);
  if kr != KERN_SUCCESS {
    return Err(napi::Error::new(
      napi::Status::GenericFailure,
      format!("IOMasterPort failed with {:?}", KernStatus::from(kr)),
    ));
  }
  let platform_device_dictionary = IOServiceMatching(b"AppleARMIODevice\0".as_ptr().cast());
  let mut iterator = std::mem::MaybeUninit::<io_iterator_t>::uninit();
  let kr = IOServiceGetMatchingServices(
    master_port,
    platform_device_dictionary,
    iterator.as_mut_ptr(),
  );
  if kr != KERN_SUCCESS {
    return Err(napi::Error::new(
      napi::Status::GenericFailure,
      format!(
        "IOServiceGetMatchingServices failed with {:?}",
        KernStatus::from(kr)
      ),
    ));
  }
  let iterator = iterator.assume_init();
  let mut platform_device_obj = IOIteratorNext(iterator);
  while platform_device_obj != 0 {
    let mut props = std::mem::MaybeUninit::<CFMutableDictionaryRef>::uninit();
    let kr = IORegistryEntryCreateCFProperties(
      platform_device_obj,
      props.as_mut_ptr(),
      kCFAllocatorDefault,
      0,
    );

    if kr != KERN_SUCCESS {
      return Err(napi::Error::new(
        napi::Status::GenericFailure,
        format!(
          "IORegistryEntryCreateCFProperties failed with {:?}",
          KernStatus::from(kr)
        ),
      ));
    }

    let properties: CFDictionary<CFString, CFType> =
      CFMutableDictionary::wrap_under_create_rule(props.assume_init()).to_immutable();

    if let Some(name) = properties.get(CFString::new("name")).downcast::<CFData>() {
      if std::str::from_utf8_unchecked(name.bytes()).to_lowercase() == "pmgr\0" {
        let mut clock_info = std::mem::MaybeUninit::<ClockInfo>::uninit();
        let mut clock_info_size = std::mem::size_of::<ClockInfo>();
        libc::sysctlbyname(
          "kern.clockrate\0".as_ptr().cast(),
          clock_info.as_mut_ptr().cast(),
          &mut clock_info_size,
          std::ptr::null_mut(),
          0,
        );

        let clock_info = clock_info.assume_init();
        if let Some(freq) = properties
          .find(CFString::new("voltage-states1-sram"))
          .and_then(|cf| cf.downcast::<CFData>())
        {
          let freq = CFDataGetBytePtr(freq.as_concrete_TypeRef());
          let freq = freq as *const u64;
          let freq = *freq;
          efficiency_core_frequency = freq / clock_info.hz as u64 / 1000_1000;
        }
        if let Some(freq) = properties
          .find(CFString::new("voltage-states5-sram"))
          .and_then(|cf| cf.downcast::<CFData>())
        {
          let freq = CFDataGetBytePtr(freq.as_concrete_TypeRef());
          let freq = freq as *const u64;
          let freq = *freq / clock_info.hz as u64 / 1000_1000;
          performance_core_frequency = freq;
        }
      }
    }
    IOObjectRelease(platform_device_obj);
    if performance_core_frequency != 0 && efficiency_core_frequency != 0 {
      break;
    }
    platform_device_obj = IOIteratorNext(iterator);
  }
  IOObjectRelease(iterator);
  IOObjectRelease(master_port);
  Ok((efficiency_core_frequency, performance_core_frequency))
});

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
static EFFICIENCY_CLUSTER_TYPE: Lazy<napi::Result<std::collections::HashMap<u32, CpuClusterType>>> =
  Lazy::new(|| unsafe {
    use std::collections::HashMap;

    use core_foundation::{
      base::{kCFAllocatorDefault, CFType, TCFType},
      data::{CFData, CFDataGetBytePtr},
      dictionary::{CFDictionary, CFMutableDictionary, CFMutableDictionaryRef},
      string::CFString,
    };
    use libc::{mach_port_t, KERN_SUCCESS, MACH_PORT_NULL};

    use crate::sys::macos_aarch64::*;

    let mut master_port: mach_port_t = MACH_PORT_NULL as _;
    let mut cluster_types = HashMap::new();

    let platform_device_dictionary = IOServiceMatching(b"IOPlatformDevice\0".as_ptr().cast());
    let kr = IOMasterPort(kIOMasterPortDefault, &mut master_port);
    if kr != KERN_SUCCESS {
      return Err(napi::Error::new(
        napi::Status::GenericFailure,
        format!("IOMasterPort failed with {:?}", KernStatus::from(kr)),
      ));
    }
    let mut iterator = std::mem::MaybeUninit::<io_iterator_t>::uninit();
    let kr = IOServiceGetMatchingServices(
      master_port,
      platform_device_dictionary,
      iterator.as_mut_ptr(),
    );
    if kr != KERN_SUCCESS {
      return Err(napi::Error::new(
        napi::Status::GenericFailure,
        format!(
          "IOServiceGetMatchingServices failed with {:?}",
          KernStatus::from(kr)
        ),
      ));
    }
    let iterator = iterator.assume_init();
    let mut platform_device_obj = IOIteratorNext(iterator);
    while platform_device_obj != 0 {
      let mut props = std::mem::MaybeUninit::<CFMutableDictionaryRef>::uninit();
      let kr = IORegistryEntryCreateCFProperties(
        platform_device_obj,
        props.as_mut_ptr(),
        kCFAllocatorDefault,
        0,
      );

      if kr != KERN_SUCCESS {
        return Err(napi::Error::new(
          napi::Status::GenericFailure,
          format!(
            "IORegistryEntryCreateCFProperties failed with {:?}",
            KernStatus::from(kr)
          ),
        ));
      }

      let properties: CFDictionary<CFString, CFType> =
        CFMutableDictionary::wrap_under_create_rule(props.assume_init()).to_immutable();

      if let Some(name) = properties
        .find(CFString::new("device_type"))
        .and_then(|d| d.downcast::<CFData>())
      {
        if std::str::from_utf8_unchecked(name.bytes()) == "cpu\0" {
          if let Some(cluster_type) = properties
            .get(CFString::new("cluster-type"))
            .downcast::<CFData>()
          {
            if let Some(cpu_id) = properties.get(CFString::new("cpu-id")).downcast::<CFData>() {
              let cpu_id = CFDataGetBytePtr(cpu_id.as_concrete_TypeRef());
              let cpu_id = cpu_id as *const u32;
              let cpu_id = *cpu_id;
              let cluster_type = std::str::from_utf8_unchecked(cluster_type.bytes());

              cluster_types.insert(
                cpu_id,
                match cluster_type {
                  "E\0" => CpuClusterType::Efficiency,
                  "P\0" => CpuClusterType::Performance,
                  _ => CpuClusterType::Unknown,
                },
              );
            }
          }
        }
      }
      IOObjectRelease(platform_device_obj);
      platform_device_obj = IOIteratorNext(iterator);
    }
    IOObjectRelease(master_port);
    Ok(cluster_types)
  });

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
#[napi::module_init]
fn pre_init() {
  let _ = &*CORE_FREQUENCY;
  let _ = &*EFFICIENCY_CLUSTER_TYPE;
}

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
enum CpuClusterType {
  Efficiency,
  Performance,
  Unknown,
}

#[napi]
pub struct Cpu {
  pub(crate) inner: &'static sysinfo::Cpu,
}

#[napi]
impl Cpu {
  #[napi]
  pub fn usage(&self) -> f32 {
    self.inner.cpu_usage()
  }

  #[cfg(not(all(target_arch = "aarch64", target_os = "macos")))]
  #[napi]
  pub fn name(&self) -> String {
    self.inner.name().to_string()
  }

  #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
  #[napi]
  pub fn name(&self) -> String {
    let name = self.inner.name();
    if let Ok(n) = name.parse::<u32>() {
      format!("cpu{}", if n != 0 { n - 1 } else { 0 })
    } else {
      name.to_string()
    }
  }

  #[napi]
  /// Cpu frequency in `MHz`
  pub fn frequency(&self) -> napi::Result<u32> {
    #[cfg(not(all(target_arch = "aarch64", target_os = "macos")))]
    {
      Ok(self.inner.frequency() as u32)
    }
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    {
      let (efficiency_core_frequency, performance_core_frequency) =
        CORE_FREQUENCY.as_ref().map_err(|err| err.clone())?;
      let efficiency_cluster_type = EFFICIENCY_CLUSTER_TYPE
        .as_ref()
        .map_err(|err| err.clone())?;

      if let Ok(c) = self.inner.name().parse::<u32>() {
        match efficiency_cluster_type.get(&(if c == 0 { 0 } else { c - 1 })) {
          Some(CpuClusterType::Efficiency) => Ok((*efficiency_core_frequency) as u32),
          Some(CpuClusterType::Performance) => Ok((*performance_core_frequency) as u32),
          _ => Ok(self.inner.frequency() as u32),
        }
      } else {
        Ok(self.inner.frequency() as u32)
      }
    }
  }

  #[napi]
  pub fn vendor_id(&self) -> String {
    self.inner.vendor_id().to_string()
  }

  #[napi]
  pub fn brand(&self) -> String {
    self.inner.brand().to_string()
  }
}
