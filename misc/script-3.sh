#!/usr/bin/env sh

cat placeholder.txt
sleep 1
wipe \
  --char-pattern rhombus \
  --char-invert true \
  --char-segments 2 \
  --char-slices 2 \
  --char-swap false \
  --color-pattern wheel \
  --color-slices 2 \
  --color-invert true \
  --color-shift true \
  --color-swap false \
  --colors cyan