use std::time::Duration;

use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, Direction, TransferType, UsbContext};

use crate::Error;

const VENDOR_ID: u16 = 0x1008;
const PRODUCT_ID: u16 = 0x1004;

/// Device information string identifiers. These strings are identified by their ASCII initials (e.g. EquipmentRecognition = 'E' = 0x45)
#[repr(u8)]
pub enum DeviceString {
    EquipmentRecognition = b'E',
    FirmwareRevision = b'F',
    ManufactureDate = b'M',
    ProductId = b'P',
    SerialNo = b'S',
    OperationTimeCounter = b'T',
}

/// How characters are displayed
pub enum CharacterMode {
    Normal = 0,
    Blinking = 5,
    Inverse = 7,
}

/// How the cursor is displayed
pub enum CursorMode {
    Off = 0,
    Blinking = 1,
    On = 2,
}

/// USB Driver for the VF60 display
pub struct Driver {
    device: DeviceHandle<Context>,
    endpoint_out: u8,
    endpoint_in: u8,
    timeout: Duration,
}

impl Driver {
    /// Find a currently connected VF60 display and connect to it.
    pub fn open() -> Result<Self, Error> {
        let device = find_vf60_device()?;
        let config = device.active_config_descriptor()?;

        let Some(interface) = config
            .interfaces()
            .flat_map(|iface| iface.descriptors())
            .next()
        else {
            return Err(Error::InterfaceNotDetected);
        };

        let bulk_out = interface
            .endpoint_descriptors()
            .find(|ep| ep.transfer_type() == TransferType::Bulk && ep.direction() == Direction::Out)
            .ok_or(Error::MissingOutEndpoint)?;
        let bulk_in = interface
            .endpoint_descriptors()
            .find(|ep| ep.transfer_type() == TransferType::Bulk && ep.direction() == Direction::In)
            .ok_or(Error::MissingInEndpoint)?;

        let mut device = device.open()?;
        device.claim_interface(interface.interface_number())?;

        let driver = Driver {
            device,
            endpoint_out: bulk_out.address(),
            endpoint_in: bulk_in.address(),
            timeout: Duration::from_secs(1),
        };
        driver.initialize()?;

        Ok(driver)
    }

    /// Set the timeout for USB operations
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn initialize(&self) -> Result<(), Error> {
        self.device
            .write_control(0x40, 11, 0, 0, &[0x03, 0x01], self.timeout)?;
        self.device
            .write_control(0x40, 11, 0, 0, &[0x01, 0x03], self.timeout)?;
        self.device
            .write_control(0x40, 11, 0, 0, &[0x00, 0x04, 0x01], self.timeout)?;

        Ok(())
    }

    /// Get a device information string
    pub fn read_device_string(&self, device_string: DeviceString) -> Result<String, Error> {
        self.device.write_bulk(
            self.endpoint_out,
            &[0x1b, 0x5b, 0x63, device_string as u8, 0x52],
            self.timeout,
        )?;

        let mut buf = [0u8; 32];
        let num_read = self
            .device
            .read_bulk(self.endpoint_in, &mut buf, self.timeout)?;
        let buf = &buf[0..num_read];
        if buf.len() < 2 {
            return Err(Error::ReadFailed);
        }

        let buf_trimmed = &buf[1..buf.len() - 1];
        Ok(String::from_utf8_lossy(buf_trimmed).to_string())
    }

    /// Write binary data directly to the device
    pub fn write(&self, data: &[u8]) -> Result<(), Error> {
        self.device
            .write_bulk(self.endpoint_out, &data, self.timeout)?;
        Ok(())
    }

    /// Print a string to the display
    pub fn print(&self, data: &str) -> Result<(), Error> {
        let data = data.as_bytes();
        self.write(data)
    }

    /// Move the cursor back without deleting the character
    pub fn backspace(&self) -> Result<(), Error> {
        self.print("\x08")
    }

    /// Move cursor down one line (but not to the left)
    pub fn line_feed(&self) -> Result<(), Error> {
        self.print("\x0a")
    }

    /// Move cursor to start of line
    pub fn carriage_return(&self) -> Result<(), Error> {
        self.print("\x0d")
    }

    /// Clear the screen and sets cursor back to (0, 0)
    pub fn clear_display(&self) -> Result<(), Error> {
        self.print("\x1b[2J")?;
        self.set_cursor_pos(0, 0)?;
        Ok(())
    }

    /// Clear all data from the current cursor position to the end of the current line
    pub fn clear_to_end(&self) -> Result<(), Error> {
        self.print("\x1b[0K")
    }

    /// Set the 0-based x and y coordinates of the cursor.
    pub fn set_cursor_pos(&self, x: u8, y: u8) -> Result<(), Error> {
        self.print(&format!("\x1b[{};{}H", y + 1, x + 1))
    }

    /// Set if and how the cursor is rendered
    pub fn set_cursor_mode(&self, mode: CursorMode) -> Result<(), Error> {
        self.print(&format!("\x1b\\?LC{}", mode as u8))
    }

    /// Set the mode of all characters that are printed following this call
    pub fn set_character_mode(&self, mode: CharacterMode) -> Result<(), Error> {
        self.print(&format!("\x1b[{}m", mode as u8))
    }

    /// Set the VFD's brightness between 0 (0%) and 5 (100%)
    pub fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
        self.print(&format!("\x1b\\?LD{}", brightness))
    }
}

fn find_vf60_device() -> Result<Device<Context>, Error> {
    let context = Context::new()?;
    let devices = context.devices()?;
    devices
        .iter()
        .find(|device| match device.device_descriptor() {
            Ok(desc) if is_vf60_device(&desc) => true,
            _ => false,
        })
        .ok_or(Error::DeviceNotDetected)
}

fn is_vf60_device(device: &DeviceDescriptor) -> bool {
    device.vendor_id() == VENDOR_ID && device.product_id() == PRODUCT_ID
}
