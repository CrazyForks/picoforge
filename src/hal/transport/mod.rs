use std::fmt;

use crate::error::PFError;
use crate::hal::types::FirmwareType;

pub mod fido;
use fido::HidTransport;

pub mod pcsc;
use pcsc::PcscTransport;

pub enum DeviceHandle {
    Fido(HidTransport),
    Rescue(PcscTransport),
}

impl fmt::Debug for DeviceHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fido(t) => f.debug_tuple("Fido").field(t).finish(),
            Self::Rescue(t) => f.debug_tuple("Rescue").field(&t.firmware_type).finish(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DeviceIdentity {
    pub vid: u16,
    pub pid: u16,
    pub product_name: String,
    pub firmware_type: FirmwareType,
}

impl DeviceHandle {
    pub fn firmware_type(&self) -> FirmwareType {
        match self {
            Self::Fido(_) => FirmwareType::Unknown,
            Self::Rescue(t) => t.firmware_type.clone(),
        }
    }

    /// Extract the inner FIDO transport, consuming the handle.
    #[allow(dead_code)]
    pub fn into_fido(self) -> Option<HidTransport> {
        match self {
            Self::Fido(t) => Some(t),
            _ => None,
        }
    }

    /// Try to discover a device via FIDO HID first, falling back to Rescue PC/SC.
    #[allow(dead_code)]
    pub fn discover() -> Result<(Self, DeviceIdentity), PFError> {
        match Self::try_fido() {
            Ok(Some((handle, identity))) => {
                log::info!("Device discovered via FIDO HID transport");
                return Ok((handle, identity));
            }
            Ok(None) => log::info!("No FIDO HID device found"),
            Err(e) => log::warn!("FIDO HID discovery error: {}", e),
        }

        match Self::try_rescue() {
            Ok(Some((handle, identity))) => {
                log::info!("Device discovered via Rescue PC/SC transport");
                return Ok((handle, identity));
            }
            Ok(None) => log::info!("No Rescue PC/SC device found"),
            Err(e) => log::warn!("Rescue PC/SC discovery error: {}", e),
        }

        Err(PFError::NoDevice)
    }

    /// Try to connect via FIDO HID transport.
    pub fn try_fido() -> Result<Option<(Self, DeviceIdentity)>, PFError> {
        let transport = HidTransport::open()?;
        let identity = DeviceIdentity {
            vid: transport.vid,
            pid: transport.pid,
            product_name: transport.product_name.clone(),
            firmware_type: FirmwareType::Unknown,
        };
        Ok(Some((Self::Fido(transport), identity)))
    }

    /// Try to connect via Rescue PC/SC transport.
    pub fn try_rescue() -> Result<Option<(Self, DeviceIdentity)>, PFError> {
        match PcscTransport::open() {
            Ok(transport) => {
                let identity = DeviceIdentity {
                    vid: 0,
                    pid: 0,
                    product_name: "Rescue Device".into(),
                    firmware_type: transport.firmware_type.clone(),
                };
                Ok(Some((Self::Rescue(transport), identity)))
            }
            Err(PFError::NoDevice) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
