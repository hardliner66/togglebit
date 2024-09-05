use std::time::Duration;

use anathema::backend::tui::TuiBackend;
use anathema::component::{Component, KeyEvent, MouseEvent};
use anathema::runtime::Runtime;
use anathema::state::{State, Value};
use anathema::templates::{Document, ToSourceKind};
use anathema::widgets::components::Context;
use anathema::widgets::Elements;

static MAIN_TEMPLATE: &str = include_str!("../main.aml");
static TOGGLEBIT_TEMPLATE: &str = include_str!("../templates/togglebit.aml");

#[derive(State)]
struct BitEnabledState {
    change_state_timeout: Value<usize>,
    enabled: Value<bool>,
}

impl BitEnabledState {
    fn new(value: bool) -> Self {
        Self {
            change_state_timeout: 0.into(),
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
        // check if we can change the value again
        if state.change_state_timeout.copy_value() == 0 {
            match key.get_char() {
                Some(' ') => {
                    state.enabled = (!state.enabled.to_bool()).into();
                    // set timeout to 15 ticks
                    state.change_state_timeout = 15.into();
                }
                _ => (),
            }
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

    fn tick(
        &mut self,
        state: &mut Self::State,
        mut _elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
        _dt: Duration,
    ) {
        let tick: usize = state.change_state_timeout.copy_value();
        state.change_state_timeout = (tick.saturating_sub(1)).into();
    }
}

fn main() {
    let doc = Document::new(MAIN_TEMPLATE);

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
            TOGGLEBIT_TEMPLATE.to_template(),
            || BitEnabled::new(),
            move || BitEnabledState::new(true),
        )
        .unwrap();

    runtime.finish().unwrap().run();
}
