use std::fs::read_to_string;

use anathema::backend::tui::TuiBackend;
use anathema::component::{Component, KeyEvent, MouseEvent};
use anathema::runtime::Runtime;
use anathema::state::{State, Value};
use anathema::templates::Document;
use anathema::widgets::components::Context;
use anathema::widgets::Elements;

#[derive(State)]
struct BitEnabledState {
    enabled: Value<bool>,
}

impl BitEnabledState {
    fn new(value: bool) -> Self {
        Self {
            enabled: value.into(),
        }
    }
}

struct BitEnabled;

impl BitEnabled {
    fn new() -> Self {
        BitEnabled
    }
}

impl Component for BitEnabled {
    type State = BitEnabledState;

    type Message = ();

    fn on_key(
        &mut self,
        key: KeyEvent,
        state: &mut Self::State,
        mut _elements: Elements<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        match key.get_char() {
            Some(' ') => {
                state.enabled = (!state.enabled.to_bool()).into();
            }
            _ => (),
        }
    }

    fn on_mouse(
        &mut self,
        mouse: MouseEvent,
        state: &mut Self::State,
        mut _elements: Elements<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        if mouse.lsb_down() {
            state.enabled = (!state.enabled.to_bool()).into();
        }
    }
}

fn main() {
    let template = read_to_string("main.aml").unwrap();

    let doc = Document::new(template);

    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_mouse()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);
    runtime
        .register_prototype(
            "togglebit",
            "templates/togglebit.aml",
            || BitEnabled::new(),
            move || BitEnabledState::new(true),
        )
        .unwrap();

    runtime.finish().unwrap().run();
}
