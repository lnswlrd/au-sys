# au-sys

Rust FFI bindings for the macOS **AudioUnit v2 (AUv2)** C API.

This crate exposes the stable AUv2 C interface from Apple's AudioToolbox framework as
idiomatic Rust FFI types, without any wrapping or abstraction layer on top.

Covered headers:
- `AudioToolbox/AUComponent.h`
- `AudioToolbox/AudioUnitProperties.h`
- `AudioToolbox/AudioComponent.h`
- `CoreAudioTypes/CoreAudioBaseTypes.h`

## Usage

Add to your `Cargo.toml`:

```toml
[target.'cfg(target_os = "macos")'.dependencies]
au-sys = "0.1"
```

## Scope

This crate provides:
- All AUv2 C types (`AudioBufferList`, `AudioStreamBasicDescription`, `AudioTimeStamp`, etc.)
- The `AudioComponentPlugInInterface` vtable struct
- Property ID constants (`kAudioUnitProperty_*`)
- Scope and selector constants
- Parameter info types and flag constants
- Minimal CoreFoundation helpers (`cf_string_create`, `cf_release`)

It does **not** provide:
- AUv3 (App Extension) bindings
- Any Objective-C or Swift interop
- A safe wrapper layer

## Platform

This crate is macOS-only. All items are gated behind `#[cfg(target_os = "macos")]`.

## License

MIT
