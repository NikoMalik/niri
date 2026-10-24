use std::time::{Duration, Instant};

use super::*;

#[test]
fn tearing_only_for_a_fullscreen_game() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    f.add_output(2, (1920, 1080));
    f.niri_focus_output(1);

    let id = f.add_client();
    let window = f.client(id).create_window();
    let surface = window.surface.clone();
    window.commit();
    f.roundtrip(id);
    let window = f.client(id).window(&surface);
    window.attach_new_buffer();
    window.set_size(100, 100);
    window.ack_last_and_commit();
    f.double_roundtrip(id);

    let (game, other) = (f.niri_output(1), f.niri_output(2));
    f.niri().set_tearing(true, Some("headless-1".to_owned()));
    assert!(
        !f.niri().tearing_now(&game),
        "a window not fullscreen tears"
    );

    let window_id = f.niri().layout.windows().next().unwrap().1.window.clone();
    f.niri().layout.set_fullscreen(&window_id, true);
    f.double_roundtrip(id);
    let window = f.client(id).window(&surface);
    window.set_size(1920, 1080);
    window.ack_last_and_commit();
    f.double_roundtrip(id);
    // (not while the view still moves to it)
    f.niri_complete_animations();
    assert!(
        f.niri().tearing_now(&game),
        "the fullscreen game doesn't tear"
    );
    assert!(!f.niri().tearing_now(&other), "another output tears");

    f.niri().set_tearing(false, None);
    assert!(!f.niri().tearing_now(&game), "tearing stays on after off");
}
