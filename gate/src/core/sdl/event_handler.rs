// Copyright 2017-2018 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashSet;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode as SdlKeyCode;

use ::{App, AppContext};
use asset_id::AppAssetId;
use input::{KeyEvent, KeyCode};
use renderer::Renderer;

pub struct EventHandler {
    pump: EventPump,
    held_keys: HashSet<KeyCode>,
}

impl EventHandler {
    pub fn new(pump: EventPump) -> EventHandler {
        EventHandler { pump, held_keys: HashSet::new() }
    }

    pub fn process_events<AS: AppAssetId, AP: App<AS>>(&mut self, app: &mut AP, ctx: &mut AppContext<AS>,
                                                       renderer: &Renderer<AS>) {
        for event in self.pump.poll_iter() {
            match event {
                Event::Quit { .. } => ctx.close(),
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if let Some(keycode) = sdl_to_gate_key(keycode) {
                        if self.held_keys.insert(keycode) {
                            app.input(KeyEvent::Pressed, keycode, ctx);
                        }
                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    if let Some(keycode) = sdl_to_gate_key(keycode) {
                        if self.held_keys.remove(&keycode) {
                            app.input(KeyEvent::Released, keycode, ctx);
                        }
                    }
                },
                Event::MouseMotion { x, y, .. } => ctx.cursor = renderer.to_app_pos(x, y),
                _ => {},
            }
            if ctx.close_requested { break; }
        }
    }
}

fn sdl_to_gate_key(sdl: SdlKeyCode) -> Option<KeyCode> {
    match sdl {
        SdlKeyCode::A => Some(KeyCode::A),
        SdlKeyCode::B => Some(KeyCode::B),
        SdlKeyCode::C => Some(KeyCode::C),
        SdlKeyCode::D => Some(KeyCode::D),
        SdlKeyCode::E => Some(KeyCode::E),
        SdlKeyCode::F => Some(KeyCode::F),
        SdlKeyCode::G => Some(KeyCode::G),
        SdlKeyCode::H => Some(KeyCode::H),
        SdlKeyCode::I => Some(KeyCode::I),
        SdlKeyCode::J => Some(KeyCode::J),
        SdlKeyCode::K => Some(KeyCode::K),
        SdlKeyCode::L => Some(KeyCode::L),
        SdlKeyCode::M => Some(KeyCode::M),
        SdlKeyCode::N => Some(KeyCode::N),
        SdlKeyCode::O => Some(KeyCode::O),
        SdlKeyCode::P => Some(KeyCode::P),
        SdlKeyCode::Q => Some(KeyCode::Q),
        SdlKeyCode::R => Some(KeyCode::R),
        SdlKeyCode::S => Some(KeyCode::S),
        SdlKeyCode::T => Some(KeyCode::T),
        SdlKeyCode::U => Some(KeyCode::U),
        SdlKeyCode::V => Some(KeyCode::V),
        SdlKeyCode::W => Some(KeyCode::W),
        SdlKeyCode::X => Some(KeyCode::X),
        SdlKeyCode::Y => Some(KeyCode::Y),
        SdlKeyCode::Z => Some(KeyCode::Z),
        SdlKeyCode::Num0 => Some(KeyCode::Num0),
        SdlKeyCode::Num1 => Some(KeyCode::Num1),
        SdlKeyCode::Num2 => Some(KeyCode::Num2),
        SdlKeyCode::Num3 => Some(KeyCode::Num3),
        SdlKeyCode::Num4 => Some(KeyCode::Num4),
        SdlKeyCode::Num5 => Some(KeyCode::Num5),
        SdlKeyCode::Num6 => Some(KeyCode::Num6),
        SdlKeyCode::Num7 => Some(KeyCode::Num7),
        SdlKeyCode::Num8 => Some(KeyCode::Num8),
        SdlKeyCode::Num9 => Some(KeyCode::Num9),
        SdlKeyCode::Right => Some(KeyCode::Right),
        SdlKeyCode::Left => Some(KeyCode::Left),
        SdlKeyCode::Down => Some(KeyCode::Down),
        SdlKeyCode::Up => Some(KeyCode::Up),
        SdlKeyCode::Return => Some(KeyCode::Return),
        SdlKeyCode::Space => Some(KeyCode::Space),
        _ => None,
    }
}
