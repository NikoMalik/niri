use smithay::output::Output;

use crate::niri::Niri;

impl Niri {
    /// `output`: the game's; none given, the active one now.
    pub fn set_tearing(&mut self, on: bool, output: Option<String>) {
        self.tearing_output = on
            .then(|| output.or_else(|| self.layout.active_output().map(|o| o.name())))
            .flatten();
    }

    /// Whether `output`'s frames may tear now: it is the game's and shows a fullscreen window.
    pub fn tearing_now(&self, output: &Output) -> bool {
        self.tearing_output
            .as_ref()
            .is_some_and(|name| *name == output.name())
            && self
                .layout
                .monitor_for_output(output)
                .is_some_and(|mon| mon.fullscreen_window_above_top_layer().is_some())
    }
}
