# YUKI
YUKI is a split diodless keyboard with a column stagger layout heavily inspired from [cantor](https://github.com/diepala/cantor).

Made with [keyberon](https://github.com/TeXitoi/keyberon).

![YUKI](imgs/Yuki-v0.1-1.jpg)

### LAYERS
- [layer 0](http://www.keyboard-layout-editor.com/#/gists/67aaf9d778e9b2ddf6e25b263cbe5ed5) (firmware dvorak)
- [layer 1](http://www.keyboard-layout-editor.com/#/gists/5f7a4db98ea4d0b959304c4fe80d1d7f) (numbers and symbols)
- [layer 2](http://www.keyboard-layout-editor.com/#/gists/3af9d73abaec154f56b99b5a6c55cf5e) (the rest of the keys of an TKL board)
- layer 3 (mouse movement and buttons)
- layer 4 (qwerty)

# BOM
- 2 Custom PCBs (files for which can be found in the yuki_pcb directory)
- 2 stm32f401 boards
- 2 TRRS connectors
- 44 cherry mx switches
- 44 cherry mx keycaps
- TRRS cable
- USB A to C cable

# Flashing the firmware
You will need rust installed so:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

cargo-binutils for stripping unused data from the rust binary and other optimizations:
```
rustup component add llvm-tools
cargo install cargo-binutils
```

dfu-util for flashing:
```
sudo pacman -S dfu-util
```

add the rust target for the MCU:
```
rustup target add thumbv7em-none-eabihf
```

then to compile the firmware:
``` 
cargo objcopy --bin yuki --release -- -O binary yuki.bin
```
for the right board add `--features right`


to flash enter dfu by holding BOOT clicking RESET and releasing BOOT and enter:
```
dfu-util -d 0483:df11 -a 0 --dfuse-address 0x08000000:leave -D yuki.bin"
```

After that to enter dfu just hold button 05 and 33 click reset and release the buttons.
