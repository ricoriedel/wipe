#!/usr/bin/env sh

cat placeholder.txt
sleep 1
wipe \
  --char-pattern circle \
  --char-invert false \
  --char-segments 3 \
  --char-slices 2 \
  --char-swap false \
  --color-pattern wheel \
  --color-slices 2 \
  --color-invert false \
  --color-shift true \
  --color-swap false \
  --colors rainbow