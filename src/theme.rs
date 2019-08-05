// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Theme keys and initial values.

use crate::piet::Color;

use crate::{Env, Key};

pub const BACKGROUND_COLOR: Key<Color> = Key::new("background_color");
pub const HOVER_COLOR: Key<Color> = Key::new("hover_color");
pub const PRESSED_COLOR: Key<Color> = Key::new("pressed_color");
pub const LABEL_COLOR: Key<Color> = Key::new("label_color");

/// An initial theme.
pub fn init() -> Env {
    Env::default()
        .adding(BACKGROUND_COLOR, Color::rgb24(0x40_40_48))
        .adding(HOVER_COLOR, Color::rgb24(0x50_50_58))
        .adding(PRESSED_COLOR, Color::rgb24(0x60_60_68))
        .adding(LABEL_COLOR, Color::rgb24(0xf0_f0_ea))
}
