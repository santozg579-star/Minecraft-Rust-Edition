# Android build notes

Target architecture: `aarch64-linux-android` (ARM64).

This scaffold documents the Rust target only. Bevy Android packaging also needs Android SDK/NDK,
a configured linker, an Android application entry point and a Gradle/Android packaging project.
Those pieces must be completed and tested before an APK can be produced.

Do not assume that `cargo build --target aarch64-linux-android` alone creates an installable APK.
