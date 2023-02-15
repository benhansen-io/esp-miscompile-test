use esp_idf_sys as _; // If using the `libstart` feature of `esp-idf-sys`, always keep this module imported

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r, g, b }
    }
}

fn next_color(curr: RGB, order: &[RGB]) -> RGB {
    let curr_idx = order
        .iter()
        .position(|c| {
            // println!("UNCOMMENT TO STOP PANIC");
            *c == curr
        })
        .unwrap();
    let next_idx = if curr_idx == order.len() - 1 {
        0
    } else {
        curr_idx + 1
    };
    order[next_idx]
}

#[no_mangle]
extern "C" fn rust_main() -> i32 {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let green: RGB = RGB::new(0, 255, 0);
    let blue: RGB = RGB::new(0, 0, 255);
    let yellow: RGB = RGB::new(255, 255, 0);

    let color_order = [yellow, blue, green];
    let mut curr = yellow;
    println!("Starting");
    loop {
        println!("curr: {:?}", curr);
        curr = next_color(curr, &color_order);
        println!("next: {:?}", curr);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
