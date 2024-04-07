use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("USB Error: {0}")]
    UsbError(#[from] rusb::Error),

    #[error("VF60 USB device not detected")]
    DeviceNotDetected,

    #[error("USB interface not detected")]
    InterfaceNotDetected,

    #[error("Could not find outbound endpoint")]
    MissingOutEndpoint,

    #[error("Could not find inbound endpoint")]
    MissingInEndpoint,

    #[error("Failed to read from USB device")]
    ReadFailed,
}
