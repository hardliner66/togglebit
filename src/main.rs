use anathema::backend::tui::TuiBackend;
use anathema::component::{Component, KeyEvent, MouseEvent};
use anathema::runtime::Runtime;
use anathema::state::{State, Value};
use anathema::templates::{Document, ToSourceKind};
use anathema::widgets::components::events::KeyState;
use anathema::widgets::components::Context;
use anathema::widgets::Elements;

static MAIN_TEMPLATE: &str = include_str!("../main.aml");
static TOGGLEBIT_TEMPLATE: &str = include_str!("../templates/togglebit.aml");

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
        if let KeyState::Press = key.state {
            match key.get_char() {
                Some(' ') => {
                    let mut enabled = state.enabled.to_mut();
                    *enabled = !*enabled;
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
            let mut enabled = state.enabled.to_mut();
            *enabled = !*enabled;
        }
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
