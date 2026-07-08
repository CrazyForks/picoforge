//! Application-level routines for interacting with the `Rescue Applet`.
//!
//! The Rescue Applet (`A0 58 3F C1 9B 7E 4F 21`) is used for out-of-band management of the Pico FIDO key,
//! allowing configuration of device parameters before FIDO provisioning occurs.
//!
//! This module delegates all APDU logic to `RescueOperations` implemented on `PcscTransport`.

pub mod constants;
pub mod ops;

use crate::error::PFError;
use crate::hal::transport::pcsc::PcscTransport;
use crate::hal::types::*;
use ops::RescueOperations;

pub fn read_device_details() -> Result<FullDeviceStatus, PFError> {
    PcscTransport::open()?.read_device_details()
}

pub fn write_config(config: AppConfigInput) -> Result<String, PFError> {
    PcscTransport::open()?.write_config(config)
}

pub fn reboot_device(to_bootsel: bool) -> Result<String, PFError> {
    PcscTransport::open()?.reboot_device(to_bootsel)
}

pub fn enable_secure_boot(lock: bool) -> Result<String, PFError> {
    PcscTransport::open()?.enable_secure_boot(lock)
}

pub fn read_led_config() -> Result<LedStatusConfig, PFError> {
    PcscTransport::open_with_aid(constants::VENDOR_LED_AID)?.read_led_config()
}

pub fn write_led_status(
    status: u8,
    color: u8,
    brightness: u8,
    steady: bool,
) -> Result<String, PFError> {
    PcscTransport::open_with_aid(constants::VENDOR_LED_AID)?
        .write_led_status(status, color, brightness, steady)
}

pub fn read_management_config() -> Result<ManagementAppConfig, PFError> {
    PcscTransport::open_with_aid(constants::MANAGEMENT_AID)?.read_management_config()
}

pub fn write_management_config(enabled_mask: u16) -> Result<String, PFError> {
    PcscTransport::open_with_aid(constants::MANAGEMENT_AID)?.write_management_config(enabled_mask)
}
