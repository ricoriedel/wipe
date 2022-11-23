#!/usr/bin/env sh

cat placeholder.txt
sleep 1
wipe \
  --char-pattern circle \
  --char-invert false \
  --char-segments 3 \
  --char-shrink 2 \
  --char-swap false \
  --color-pattern wheel \
  --color-segments 2 \
  --color-invert false \
  --color-shift true \
  --color-swap false \
  --colors rainbow