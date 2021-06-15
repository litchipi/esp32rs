### Setup
Build Rust for xtensa support:
```
git clone https://github.com/MabezDev/rust-xtensa
./configure --experimental-targets=Xtensa
./x.py build --stage 2
```
Note:	Will take a LOT of time
Note:	I recomment to turn off ninja in config.toml `ninja = false`

#### Download the toolchain from Espressif ((Link)[https://github.com/espressif/crosstool-NG/releases])
WARNING:	Take the esp32 one, not esp32s3 one
```
mkdir ./esp32_toolchain/
tar -xzf ~/Downloads/xtensa-esp32-elf-gcc8_4_0-esp-2020r3-linux-amd64.tar.gz -C ./esp32_toolchain/
```

#### Install tools for building / flashing
```
cargo install cargo-xbuild
cargo install cargo-espflash
```

#### Setup bash env to cross compile.
Add this to a file `setup.sh` (or the name you want)

```
CUSTOM_RUST_BUILD=/path/to/rust-xtensa

export RUST_BACKTRACE=1

export XARGO_RUST_SRC=$CUSTOM_RUST_BUILD/library
export RUSTC=$CUSTOM_RUST_BUILD/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
export RUSTDOC=$CUSTOM_RUSTC/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc

export PATH="$PATH:/path/to/esp32_toolchain/xtensa-esp32-elf/bin"
```

### Build

Setup cross compilation environment if not done already
```
source setup.sh
```

Then build the binary for the target:
```
cargo xbuild --features="xtensa-lx-rt/lx6,xtensa-lx/lx6" --target="xtensa-esp32-none-elf"
```

### Flash the board
Flash the ESP32 using the binary
```
cargo espflash --chip esp32 --speed 460800 --features="xtensa-lx-rt/lx6,xtensa-lx/lx6" /dev/ttyUSB0
```

### Tips
To build and flash the target more easily, add the aliases at the end of `setup.sh`
```
alias esp32_build='cargo xbuild --features="xtensa-lx-rt/lx6,xtensa-lx/lx6" --target="xtensa-esp32-none-elf'
alias esp32_flash='cargo espflash --chip esp32 --speed 460800 --features="xtensa-lx-rt/lx6,xtensa-lx/lx6" /dev/ttyUSB0'
```

To know if your environment is set up for cross compile, add to `setup.sh`
```
if [ -z ${CROSSCOMPILE_SET+x} ]; then
	export PS1="$PS1\033[95mESP32>\033[0m "
	export CROSSCOMPILE_SET=1
fi
```
