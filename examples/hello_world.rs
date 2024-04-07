fn main() -> Result<(), vf60::Error> {
    let vf60 = vf60::Driver::open()?;
    println!(
        "Detected VF60 display: {} (serial no {}, firmware {}, manufactured {})",
        vf60.read_device_string(vf60::DeviceString::EquipmentRecognition)?,
        vf60.read_device_string(vf60::DeviceString::SerialNo)?,
        vf60.read_device_string(vf60::DeviceString::FirmwareRevision)?,
        vf60.read_device_string(vf60::DeviceString::ManufactureDate)?
    );

    vf60.clear_display()?;
    vf60.print("Hello, world!")?;

    Ok(())
}
