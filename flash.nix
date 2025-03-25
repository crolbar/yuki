pkgs: [
  (pkgs.writers.writeBashBin "flash" ''
    feats=""

    if [[ "$1" == "right" ]]; then
        feats="$feats right"
    elif [[ "$1" == "left" ]]; then
        feats=$feats
    else
        echo "incorrect board part, supported: left/right"
        exit
    fi

    if [[ "$2" == "-oled" || "$2" == "--oled" || "$2" == "oled" ]]; then
        feats="$feats oled"
    fi

    echo $feats

    cargo objcopy --features "$feats" --bin yuki --release -- -O binary yuki.bin

    echo "press enter after the reset button has been pressed ..."
    read

    sudo dfu-util -d 0483:df11 -a 0 --dfuse-address 0x08000000:leave -D yuki.bin
  '')
]
