fn main() {
    println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libis_odd.so");
}
