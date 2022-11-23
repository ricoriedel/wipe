#!/usr/bin/env sh

cat placeholder.txt
sleep 1
wipe \
  --char-pattern wheel \
  --char-invert false \
  --char-segments 2 \
  --char-shrink 2 \
  --char-swap false \
  --color-pattern circle \
  --color-segments 4 \
  --color-invert false \
  --color-shift false \
  --color-swap false \
  --colors dark-magenta