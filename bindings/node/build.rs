extern crate napi_build;

fn main() {
    // Configure napi build
    napi_build::setup();

    // Use stable ABI to avoid linking Python development headers
    println!("cargo:rustc-env=PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1");

    // Try to find Python library and link it explicitly
    // This allows PyO3 symbols to be resolved at runtime
    let python_lib = "/home/linuxbrew/.linuxbrew/opt/python@3.14/lib";
    println!("cargo:rustc-link-search={}", python_lib);

    // Try linking the full path to libpython
    // The -l flag should work with the search path set above
    println!("cargo:rustc-link-arg=-lpython3.14");

    // Allow undefined symbols to not break the build
    println!("cargo:rustc-link-arg=-Wl,--allow-shlib-undefined");
}
