use anathema::{
    backend::tui::TuiBackend,
    component::{Component, Event, KeyCode, KeyEvent, MouseEvent, MouseState},
    runtime::{GlobalEvents, Runtime},
    state::{State, Value},
    templates::{Document, ToSourceKind},
    widgets::{
        components::{events::KeyState, Context},
        Elements,
    },
};
use clap::Parser;
use rand::prelude::*;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    carnage: bool,
}

static MAIN_TEMPLATE: &str = include_str!("../main.aml");
static TOGGLEBIT_TEMPLATE: &str = include_str!("../templates/togglebit.aml");
static TOGGLE_0: &str = include_str!("../toggle0.txt");
static TOGGLE_1: &str = include_str!("../toggle1.txt");

#[derive(State)]
struct BitEnabledState {
    toggle_text: Value<String>,
}

impl BitEnabledState {
    fn new(value: bool) -> Self {
        Self {
            toggle_text: value
                .then(|| TOGGLE_1.to_owned())
                .unwrap_or_else(|| TOGGLE_0.to_owned())
                .into(),
        }
    }
}

struct BitEnabled {
    enabled: bool,
    rng: ThreadRng,
    carnage: bool,
    clicks: usize,
    degradation_threshold: f32,
    toggle0: Vec<char>,
    toggle1: Vec<char>,
}

fn randomize_chars(carnage: bool, rng: &mut ThreadRng, text: &mut Vec<char>) {
    let mut index = rng.gen_range(0..text.len());
    while text[index] == '\n' && !carnage {
        index = rng.gen_range(0..text.len());
    }
    let value = text[index] as u32;
    let mut new_value = None;
    while let None = new_value {
        let next_value = value ^ (1 << rng.gen_range(0..32));
        if let Ok(a) = char::try_from(next_value) {
            if !carnage && a == '\n' {
                continue;
            }
            new_value = Some(a);
        }
    }
    text[index] = new_value.unwrap();
}

impl BitEnabled {
    fn new(enabled: bool, carnage: bool) -> Self {
        BitEnabled {
            rng: thread_rng(),
            enabled,
            carnage,
            toggle0: TOGGLE_0.chars().collect(),
            toggle1: TOGGLE_1.chars().collect(),
            clicks: 0,
            degradation_threshold: 100.0,
        }
    }

    fn change_state(&mut self, state: &mut BitEnabledState) {
        self.clicks = self.clicks.wrapping_add(1);

        self.enabled = !self.enabled;

        if self.rng.gen::<f32>() < self.clicks as f32 / self.degradation_threshold {
            if self.enabled {
                randomize_chars(self.carnage, &mut self.rng, &mut self.toggle1);
            } else {
                randomize_chars(self.carnage, &mut self.rng, &mut self.toggle0);
            }
        }

        let mut text = state.toggle_text.to_mut();
        if self.enabled {
            *text = self.toggle1.iter().collect();
        } else {
            *text = self.toggle0.iter().collect();
        }
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
                    self.change_state(state);
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
        if let MouseState::Down(anathema::component::MouseButton::Left) = mouse.state {
            self.change_state(state);
        }
    }
}

struct Global;

impl GlobalEvents for Global {
    fn handle(
        &mut self,
        event: anathema::component::Event,
        _elements: &mut Elements<'_, '_>,
        _ctx: &mut anathema::prelude::GlobalContext<'_>,
    ) -> Option<anathema::component::Event> {
        match event {
            anathema::component::Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => Some(Event::Stop),
            e => Some(e),
        }
    }
}

fn main() {
    let Args { carnage } = Args::parse();
    let doc = Document::new(MAIN_TEMPLATE);

    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_mouse()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    let enabled: bool = random();

    let mut runtime = Runtime::builder(doc, backend).global_events(Global);

    runtime
        .register_prototype(
            "togglebit",
            TOGGLEBIT_TEMPLATE.to_template(),
            move || BitEnabled::new(enabled, carnage),
            move || BitEnabledState::new(enabled),
        )
        .unwrap();

    runtime.finish().unwrap().run();
}
