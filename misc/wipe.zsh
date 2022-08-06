alias clear='wipe'

_wipe() {
  wipe
  zle reset-prompt
}
zle -N _wipe
bindkey '^l' _wipe