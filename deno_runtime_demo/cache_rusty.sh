#!/bin/bash

export RUSTY_V8_MIRROR=$HOME/.cache/rusty_v8

for REL in v0.71.0 v0.72.0 v0.73.0 v0.74.0 v0.75.0 v0.76.0 v0.77.0; do
  mkdir -p $RUSTY_V8_MIRROR/$REL
#   for FILE in \
#     # librusty_v8_debug_x86_64-unknown-linux-gnu.a \
#     librusty_v8_release_x86_64-unknown-linux-gnu.a \
#   ; do
    FILE=librusty_v8_release_x86_64-unknown-linux-gnu.a
    if [ ! -f $RUSTY_V8_MIRROR/$REL/$FILE ]; then
        mkdir $RUSTY_V8_MIRROR/$REL
        wget -O $RUSTY_V8_MIRROR/$REL/$FILE https://github.com/denoland/rusty_v8/releases/download/$REL/$FILE
    fi
#   done
done