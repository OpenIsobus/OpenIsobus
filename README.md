# OpenIsobus

An open implementation of the ISO11783 standard written in [Rust](https://www.rust-lang.org/).

**Currently in the proces of splitting the big `open-isobus` crate into multiple independent crates**

**Disclaimer**: Since this crate is in active development as of 11-2022, API changes may appear at any time. Please consider this if you want to use this crate.

## Installation

- Use `open-isobus = { git = "https://github.com/OpenIsobus/OpenIsobus.git" }` as a dependency in cargo.toml of your project

or

- Run one of the [examples](#examples) given below.

**Note**; The dev-dependency `embassy` requires the nightly toolchain; 
- Install with `rustup toolchain install nightly`
- Set as default with `rustup default nightly`

### Features
- `std` Use the rust standard library, don't use for `no_std` compatibility.
- `default` = `["mock_can_driver", "log_can"]`
- `win32` = `["std", "peak_can_driver", "log_can"]` Use on windows with Peak CAN Driver.
- `cm4` = `["std", "socket_can_driver"]` Use on the Raspberry Pi 4 or the Raspberry Pi CM4.
### Can drivers
- `peak_can_driver` Use PCANBasic.
- `socket_can_driver` Use Linux socket_can.
- `mock_can_driver` Use a mock implementation to prevent errors.
### Logging
- `log_can`, log all send CAN messages and incomming messages addressed to us.
- `log_all_can`, log all send CAN messages and all incomming network messages.

## Examples
To try the library, download the git repository and run one of the following cargo commands:
- `cargo run --example embassy` To use the embassy library for embedded multi threading.
- `cargo run --example no_std` To use a single threaded implementation. **NOTE: As this is a demo, it still uses std for timekeeping**
- `cargo run --example threads` To use std::thread for multi threading.

## License / Terms of Usage

The source code of this project is licensed under the MIT/Apache-2.0 license. This implies that you are free to use, share, and adapt it. However, please give appropriate credit by citing the project.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the MIT/Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Contact

If you have problems using the software, find mistakes, or have general questions please use the [issue tracker](https://github.com/OpenIsobus/OpenIsobus/issues) to contact us.
