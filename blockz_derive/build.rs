//! build.rs

fn main() {
    let debug = option_env!("BLOCKZ_DERIVE_DEBUG");
    if debug.is_some() {
        println!("cargo:rustc-cfg=feature=\"debug_macro_errors\"");
    }
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=BLOCKZ_DERIVE_DEBUG");
}
