# ch58x-hal

[![Github Actions][github-workflow]][homepage]
[![Crates.io][badge-license]][crates]
[![Crates.io][badge-version]][crates]
[![docs.rs][badge-docsrs]][docsrs]

[github-workflow]: https://img.shields.io/github/actions/workflow/status/ch32-rs/ch58x-hal/rust.yml?style=for-the-badge
[badge-license]: https://img.shields.io/crates/l/ch58x-hal?style=for-the-badge
[badge-version]: https://img.shields.io/crates/v/ch58x-hal?style=for-the-badge
[badge-docsrs]: https://img.shields.io/docsrs/ch58x-hal?style=for-the-badge
[crates]: https://crates.io/crates/ch58x-hal
[docsrs]: https://docs.rs/ch58x-hal
[homepage]: https://github.com/ch32-rs/ch58x-hal

HAL for the CH58x RISC-V BLE microcotrollers from WCH.

This crate is under random and active development. DO NOT USE in production.

This should be the reference hal implementation for CH57x, CH58x, CH59x.

## How to setup Rust embedded environment on Windows 7 Machine:
 - Download Rust installer from here https://rustup.rs/
 - Use option 2 and enter default host triple as "x86_64-pc-windows-gnu" and use enter key for all other options.
 - Now use option 1 to proceed with installation(default). This will use default host triple "x86_64-pc-windows-gnu".
 ###   Now use following 3 commands in project root directory:
 - cargo install cargo-binutils
 - rustup component add llvm-tools
 - cargo objcopy --release --example blinky -- -O ihex blinky.hex
 - Use official WCHISP tool from WCH to flash *.hex file. Tool can be downloaded from here https://www.wch-ic.com/downloads/WCHISPTool_Setup_exe.html
### I used this very inexpensive BLE developement board from WeAct Studio - https://github.com/WeActStudio/WeActStudio.WCH-BLE-Core
 - https://www.aliexpress.us/item/3256804598673258.html?spm=a2g0o.store_pc_groupList.8148356.54.7d8a7b26VcHolM&pdp_npi=4%40dis%21USD%21US%20%242.28%21US%20%240.99%21%21%2116.24%217.05%21%402103250717043122710356394e0c03%2112000034304021507%21sh%21US%212368685392%21&gatewayAdapt=glo2usa

## Features

- Basic: clock init, delay, interrupt, etc.
- Dedicated runtime: interrupt table, hardware stack push, highcode support, critical section implementation
- embassy support
  - time driver with SysTick, defaults to 32KHz tick
  - about 7k flash rom overhead
- GPIO, with async support
- UART, basic blocking tx, rx
- RTC, with datetime support
- SysTick delay (conflicts with embassy time driver)
- I2C
- ADC, with Temperature sensor, VBAT sensor
- SPI
- libISP ROM functions
- Basic BLE library support

## Usage

Refer `Cargo.toml` and `examples` directory.

## Notes

- `UNDOCUMENTED:` tags in code comments means the information is not from official documents.

## References

- [Slappy2022/ch58x-hal](https://github.com/Slappy2022/ch58x-hal)
- [Slappy2022/ch58x-ble-rt](https://github.com/Slappy2022/ch58x-ble-rt)
