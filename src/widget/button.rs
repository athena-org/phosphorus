// Copyright 2015 The Athena Developers.
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

use std;
use gfx;
use widget;
use render;

/// Object that allows you to build button widgets.
pub struct ButtonBuilder {
    text: String,
    size: [i32; 2]
}

impl ButtonBuilder {
    /// Initializes a new `ButtonBuilder` with default values.
    pub fn new() -> ButtonBuilder {
        ButtonBuilder {
            text: String::default(),
            size: [80, 20]
        }
    }

    /// Requests a specific text content for the widget.
    pub fn with_text(mut self, text: &str) -> ButtonBuilder {
        self.text = String::from(text);
        self
    }

    /// Builds the widget.
    pub fn build_boxed<R: gfx::Resources>(self) -> Box<Button<R>> {
        Box::new(Button {
            text: self.text,
            size: self.size,
            _r: std::marker::PhantomData
        })
    }
}

pub struct Button<R: gfx::Resources> {
    text: String,
    size: [i32; 2],

    _r: std::marker::PhantomData<R>
}

impl<R: gfx::Resources> widget::Widget<R> for Button<R> {
    fn render(
        &self, renderer: &mut render::Renderer<R>,
        prev_area: &render::RenderArea, offset: &mut render::RenderOffset)
    {
        let pos = [
            (prev_area.position[0] + offset.position[0]),
            (prev_area.position[1] + offset.position[1])];

        renderer.render_rect_flat(pos, self.size, [0.29, 0.29, 0.29]); // Highlighted: 0.33
        renderer.render_text([pos[0] + 4, pos[1] + 1], "Test");
    }
}
