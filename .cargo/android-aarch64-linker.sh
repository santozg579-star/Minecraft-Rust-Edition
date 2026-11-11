#!/bin/sh
NDK_SYSROOT=/opt/android-sdk/ndk/27.2.12479018/toolchains/llvm/prebuilt/linux-x86_64/sysroot

exec /data/data/com.termux/files/usr/bin/clang \
  --target=aarch64-linux-android24 \
  --sysroot="$NDK_SYSROOT" \
  -fuse-ld=lld \
  -L/data/data/com.termux/files/usr/lib \
  "$@"
