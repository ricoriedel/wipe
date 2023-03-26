#!/usr/bin/env fish

function clear
    command wipe
end

bind \cl 'wipe; commandline -f repaint'