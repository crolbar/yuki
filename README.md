<h1 align="center">YUKI</h1>

<div align="center">
YUKI is a split diodless keyboard with a column stagger layout heavily inspired by <a href="https://github.com/diepala/cantor">cantor</a>.
</div>
<br/><br/>

![YUKI](.github/assets/YUKI-v0.2-1.jpg)
Made with [Keyberon](https://github.com/TeXitoi/keyberon)

<br/>

### Layers
| Layer                                                                                     | Description              |
|-------------------------------------------------------------------------------------------|--------------------------|
| [layer 0](http://www.keyboard-layout-editor.com/#/gists/67aaf9d778e9b2ddf6e25b263cbe5ed5) | Firmware Dvorak          |
| [layer 1](http://www.keyboard-layout-editor.com/#/gists/5f7a4db98ea4d0b959304c4fe80d1d7f) | Numbers & Symbols        |
| [layer 2](http://www.keyboard-layout-editor.com/#/gists/3af9d73abaec154f56b99b5a6c55cf5e) | Less used keys & Macros  |
| layer 3                                                                                   | Mouse movement & buttons |
| layer 4                                                                                   | Qwerty                   |

<br/>

### Features

- 44 keys, v0.1 - Cherry MX & v0.2 - Kailh Choc V1 switches
- Uses only 1U keycaps
- TRRS connection is used for communication between the two halves
- 2 USB-C connectors, with a choice from which the keyboard should send keystrokes
- Mouse control
- OLED display (not installed in the v0.2 picture)

<br/>


# Bill of Materials

| Product       | Qty | Note                                                                                                                                                                                                        |
|---------------|-----|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| YUKI PCB      | 2   | [KiCad](https://kicad.org/) project files can be found in `yuki_pcb` directory. I ordered them from [JLCPCB](https://jlcpcb.com). They are reversible so you will use the same pcb for both left and right. |
| STM32F401CCU6 | 2   | Can be found on aliexpress. I have ordered mine from a local vendor.                                                                                                                                        |
| TRRS jacks    | 2   | `PJ-320A`                                                                                                                                                                                                   |
| TRRS cable    | 1   | 3.5mm jack cable, with 3 stripes.                                                                                                                                                                           |
| Switches      | 44  | Cherry MX for v0.1 and Kailh Choc V1 for v0.2. Ordered from [splitkb](https://splitkb.com/collections/switches-and-keycaps/products/kailh-low-profile-choc-switches)                                        |
| Keycaps       | 44  | 1U keycaps. Ordered from [splitkb](https://splitkb.com/collections/switches-and-keycaps/products/blank-mbk-choc-low-profile-keycaps).                                                                       |


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


to flash enter dfu by holding BOOT clicking RESET and releasing BOOT and enter (can be tricky):
```
dfu-util -d 0483:df11 -a 0 --dfuse-address 0x08000000:leave -D yuki.bin"
```

After that to enter dfu just hold button 05 and 33 click reset and release the buttons.

### Nix

if you are on NixOS you could use the devShell:

enter the dev shell:
```
nix develop
```

and use the flash script that can be found in `flash.nix`
```
flash right
```
