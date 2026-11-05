fn main() {
    // Tell MSVC link.exe to use our custom function as the binary entry point
    println!("cargo:rustc-link-arg-bins=/ENTRY:main_crt_startup");
    // Ensure no terminal window flashes when executed
    println!("cargo:rustc-link-arg-bins=/SUBSYSTEM:WINDOWS");
}