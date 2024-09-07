# vf60-rs

A driver for Fujitsu VF60 POS displays based on [rusb](https://github.com/a1ien/rusb).

## Features

-   Print characters to screen at any position (supports different cursor and character render styles)
-   Read device information
    -   Equipment recognition
    -   Firmware revision
    -   Manufacturing date
    -   Product ID
    -   Serial number
    -   Operation time
-   Control display brightness

## Quickstart

```rust
let vf60 = vf60::Driver::open()?;
vf60.clear_display()?;
vf60.print("Hello, world!")?;
```

## Examples

-   [Hello world](https://github.com/Twometer/vf60-rs/blob/main/examples/hello_world.rs)
-   [Read device info](https://github.com/Twometer/vf60-rs/blob/main/examples/device_info.rs)

## License

Licensed under the [MIT License](https://github.com/Twometer/vf60-rs/blob/main/LICENSE)
