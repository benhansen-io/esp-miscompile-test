use esp_idf_sys as _; // If using the `libstart` feature of `esp-idf-sys`, always keep this module imported

#[no_mangle]
extern "C" fn rust_main() -> i32 {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello world from Rust!");

    42
}
