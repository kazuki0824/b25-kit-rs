#![allow(unused_imports)]
use crate::channels::Channel;
use std::error::Error;
use futures::AsyncRead;

#[cfg(target_os = "windows")]
mod Win_Bon;
#[cfg(target_os = "linux")]
mod linux;

pub enum DeviceKind {
    LinuxChardev,
    WinBon,
}

pub trait UnTuned {
    fn open(path: &str) -> Result<Device, Box<dyn Error>>;
    fn tune(self, channel: Channel, offset_k_hz: i32) -> TunedDevice;
}
pub trait Tuned {
    fn signal_quality(&self) -> f64;
    fn set_lnb(&self) -> Result<i8, String>;
    fn open(&self) -> Box<dyn AsyncRead + Unpin>;
}


#[cfg(target_os = "linux")]
pub struct Device {
    pub handle: std::os::unix::io::RawFd,
    kind: DeviceKind,
}
#[cfg(target_os = "linux")]
pub struct TunedDevice {
    pub d: Device,
    pub channel: Channel,
}

#[cfg(target_os = "windows")]
pub struct Device {
    bon_driver_path: String,
    dll_imported: BonDriver,
    pub kind: DeviceKind,
    pub interface: IBon
}
#[cfg(target_os = "windows")]
pub struct TunedDevice {
    pub d: Device,
    pub channel: Channel,
}
