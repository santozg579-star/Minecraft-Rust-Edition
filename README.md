# Minecraft Rust Edition — Android prototype 0.2

A small Rust + Bevy voxel-world prototype intended as a starting point for Android development.

## Current prototype scope
- Procedural block terrain generated from a simple height function.
- First-person 3D camera and directional lighting.
- Keyboard movement and mouse-look for desktop development/testing.
- Bevy project structure ready for Android-target work.

## Important limitations
This is a development scaffold, **not a finished Minecraft clone and not a compiled APK**.
Touchscreen controls, block breaking/placing, optimized chunk meshing, save/load, multiplayer,
and polished mobile UI still need implementation. The current input systems are desktop-oriented.

## Requirements
- Rust toolchain and Cargo
- Android SDK/NDK for Android builds
- Bevy-compatible Android build environment

## Run on desktop
```bash
cargo run
```
Hold the left mouse button to look around. Use WASD to move, Space to rise, and Left Shift to descend.

## Android target
Install the Android Rust target:
```bash
rustup target add aarch64-linux-android
```
Configure Android SDK/NDK and the linker for your environment before building. An Android build
requires a correctly configured Bevy Android application entry point and packaging/signing
workflow; this repository does not include a prebuilt APK.

## Suggested next milestones
1. Replace per-block entities with chunk meshes.
2. Add touch joystick and look area.
3. Add raycast-based block selection, breaking, and placement.
4. Add world save/load.
5. Profile on a physical Android device.
