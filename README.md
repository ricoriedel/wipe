# wipe
Wipe the content of your terminal with a random animation.

This is a fancy alternative to the `clear` command.
It plays randomly generated beautiful animations.

### Build & install
Building this project requires Rust and Cargo to be installed.
```shell
cargo build --release
```
```shell
cp ./target/release/wipe /usr/local/bin
```

### Shell Integration
There are scripts for different shells which can be sourced to replace `clear` and `CTRL+L` with this program.
The scripts are located in `misc/shell/`.

| Shell | Script       |
|:------|:-------------|
| ZSH   | `wipe.zsh`   |
| Fish  | `wipe.fish`  |

### Arch Linux
There is an [AUR package](https://aur.archlinux.org/packages/wipe-term) called `wipe-term`.
The scripts can be integrated as follows:

#### ZSH
Put this into your `.zshrc`:
```shell
source /usr/share/zsh/plugins/wipe/wipe.zsh
```

#### Fish
The package will place the script under `/usr/share/fish/vendor_conf.d/` 
which will be sourced by `fish` with no further configuration required.

## Showcase
[![Animation 1](misc/res/rec-1.gif)]()
[![Animation 2](misc/res/rec-2.gif)]()
[![Animation 3](misc/res/rec-3.gif)]()