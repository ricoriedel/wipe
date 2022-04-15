# Wipe
Wipe your terminal with a smooth animation.

This is the perfect program for you, if you like `clear` but want to add an unnecessary animation.

Download options can be found in the [release tab](https://github.com/ricoriedel/wipe/releases).

## Configuration
All configuration is done using command line parameters.
For a list of parameters, execute `wipe -h`.
Note that some parameters like `--color` can be specified multiple times with different values.

If you want a persistent solution, you can add an alias to your `.bashrc` equivalent.
```shell
# Persistent config
alias wipe='wipe -c red'

# Replace clear with wipe
alias clear='wipe'
```

If you are using `ZSH` as your shell, you can add a keyboard shortcut like this:
```shell
# Bind wipe to CTRL+W
wipe-zle() { 
  wipe
  zle reset-prompt 
}
zle -N wipe-zle
bindkey '^w' wipe-zle
```

## Showcase
[![Circle](doc/circle.gif)]()
[![Rhombus](doc/rhombus.gif)]()
[![Rotation](doc/sonar.gif)]()
