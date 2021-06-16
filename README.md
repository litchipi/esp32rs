# ESP32rs
Simple project to use Rust to program the Lora32 board from Heltec, got from [Hackerbox #0066](https://hackerboxes.com/products/hackerbox-0066-radio-star)

Uses extensively already made projects (thanks to esp-rs / MabezDev) and made to play.

## How to use
### Setup
Build Rust for xtensa support:
```bash
git clone https://github.com/MabezDev/rust-xtensa
./configure --experimental-targets=Xtensa
./x.py build --stage 2
```
Note:	Will take a LOT of time
Note:	I recomment to turn off ninja in config.toml `ninja = false`

#### Download the toolchain from Espressif ([Link](https://github.com/espressif/crosstool-NG/releases))
WARNING:	Take the esp32 one, not esp32s3 one
```bash
mkdir ./esp32_toolchain/
tar -xzf ~/Downloads/xtensa-esp32-elf-gcc8_4_0-esp-2020r3-linux-amd64.tar.gz -C ./esp32_toolchain/
```

#### Install tools for building / flashing
```bash
cargo install cargo-xbuild
cargo install cargo-espflash
```

#### Setup bash env to cross compile.
Add this to a file `setup.sh` (or the name you want)

```bash
CUSTOM_RUST_BUILD=/path/to/rust-xtensa

export RUST_BACKTRACE=1

export XARGO_RUST_SRC=$CUSTOM_RUST_BUILD/library
export RUSTC=$CUSTOM_RUST_BUILD/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
export RUSTDOC=$CUSTOM_RUSTC/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc

export PATH="$PATH:/path/to/esp32_toolchain/xtensa-esp32-elf/bin"
```

### Build

Setup cross compilation environment if not done already
```bash
source setup.sh
```

Then build the binary for the target:
```bash
cargo xbuild --features="xtensa-lx-rt/lx6,xtensa-lx/lx6" --target="xtensa-esp32-none-elf"
```

### Flash the board
Flash the ESP32 using the binary
```bash
cargo espflash --chip esp32 --speed 460800 --features="xtensa-lx-rt/lx6,xtensa-lx/lx6" /dev/ttyUSB0
```

### Tips
To build and flash the target more easily, add the aliases at the end of `setup.sh`
```bash
alias esp32_build='cargo xbuild --features="xtensa-lx-rt/lx6,xtensa-lx/lx6" --target="xtensa-esp32-none-elf"
alias esp32_flash='cargo espflash --chip esp32 --speed 460800 --features="xtensa-lx-rt/lx6,xtensa-lx/lx6" /dev/ttyUSB0'
```

To reduce the size of the output binary, strip the binary from un-necessary data:
```bash
alias esp32_strip='esp32_toolchain/xtensa-esp32-elf/bin/xtensa-esp32-elf-strip'
```

The command `esp32_strip target/xtensa-esp32-none-elf/release/esp32rs` resulted in the reduction of
the binary size from `493k` to `43k` on the blinky example.

To know if your environment is set up for cross compile, add to `setup.sh`
```bash
if [ -z ${CROSSCOMPILE_SET+x} ]; then
export PS1="$PS1\033[95mESP32>\033[0m "
	export CROSSCOMPILE_SET=1
fi
```

## Examples
In order to easily test a lot of different programs inside the same code, the file `main.rs` hosts everything statically defined (like the CPU freq for exemple).
In it, a `trait Algo` is defined, with a `init` function (initialize algorithm) and a `loop_fct` function (called over and over again), Arduino-style.
An algorithm is imported as a struct implementing the `trait Algo`, imported as a mod inside `main.rs` and called.

### Blinky
Simply blinks a LED

## TODO
### General
- Select program to launch based on Cargo.toml configuration
- Create a UART communication to display messages
	- Optionnally from Cargo.toml
	- On debug build only ?

### Blinky
- Validate that we get a second when delaying of CORE_HZ
- Simplify LED pin pick process

### Other projects
- Oled display experimentation
- Wifi experimentation
- BLE experimentation
- LoRa experimentation
