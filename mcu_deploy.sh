cargo build --release
cargo objcopy --bin black-pill-stm32f401 --target thumbv7m-none-eabi --release -- -O binary stm32f401cd.bin
st-flash erase
st-flash write stm32f401cd.bin 0x8000000