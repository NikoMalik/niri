use std::sync::atomic::{AtomicU32, Ordering};

static PIXEL: AtomicU32 = AtomicU32::new(0);

pub fn pixel() -> f32 {
    f32::from_bits(PIXEL.load(Ordering::Relaxed))
}

pub fn set(pixel: f32) {
    PIXEL.store(pixel.max(0.).to_bits(), Ordering::Relaxed);
}

pub fn over(mut shadow: niri_config::Shadow) -> niri_config::Shadow {
    if pixel() > 0. {
        shadow.on = true;
    }
    shadow
}
