# windows_screen_off

An ultra-minimal (~3 KB), instant display standby utility for Windows, written in modern Rust (`edition = "2024"`).

Unlike standard batch scripts, PowerShell snippets, or bulky C# utilities that flash a console window or pull in heavy runtimes, `windows_screen_off` 
compiles to a standalone, console-less Windows executable with zero runtime dependencies beyond Windows native system DLLs.

---

## Features

- **Tiny footprint:** Compiles to ~3 KB.
- **Zero console flash:** Configured directly with `/SUBSYSTEM:WINDOWS` so no command-prompt window flickers when triggered.
- **Freestanding Rust:** Built with `#![no_std]` and `#![no_main]`—completely strips out the Rust standard library and runtime overhead.
- **Instant standby:** Broadcasts `WM_SYSCOMMAND` with `SC_MONITORPOWER` to put all connected displays into standby power mode immediately.

---

## How It Works

1. **Direct Win32 Interop:** Uses `windows-sys` with minimal features to emit raw FFI calls to `SendMessageW` and `ExitProcess`.
2. **Custom Entry Point:** Bypasses standard C runtime initialization via a custom `main_crt_startup` entry point linked directly by MSVC's `link.exe`.
3. **No-Standard Optimization:**
   - `#![no_std]` and `#![no_main]` remove `std` runtime tables, stack overflow probes, and panic unwinding machinery.
   - Size-optimized profile: `opt-level = "z"`, `codegen-units = 1`, aggressive Link-Time Optimization (`lto = true`), and full symbol stripping (`strip = true`).

---

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) (2024 edition support)
- Visual Studio C++ Build Tools (MSVC `link.exe` toolchain)

### Build Command

Clone the repository and compile using the release profile:

```cmd
git clone https://github.com/edrf0/windows_screen_off.git
cd windows_screen_off
cargo build --release
