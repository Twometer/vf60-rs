fn main() -> Result<(), vf60::Error> {
    let vf60 = vf60::Driver::open()?;

    println!("== VF60 display information ==");
    println!(
        "Equipment recognition: {}",
        vf60.read_device_string(vf60::DeviceString::EquipmentRecognition)?
    );
    println!(
        "Firmware revision: {}",
        vf60.read_device_string(vf60::DeviceString::FirmwareRevision)?
    );
    println!(
        "Manufacture date: {}",
        vf60.read_device_string(vf60::DeviceString::ManufactureDate)?
    );
    println!(
        "USB Product ID: {}",
        vf60.read_device_string(vf60::DeviceString::ProductId)?
    );
    println!(
        "Serial number: {}",
        vf60.read_device_string(vf60::DeviceString::SerialNo)?
    );
    println!(
        "Operation time counter: {}",
        vf60.read_device_string(vf60::DeviceString::OperationTimeCounter)?
    );

    Ok(())
}
