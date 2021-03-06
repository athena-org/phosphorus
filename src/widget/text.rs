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
use std::default::Default;
use gfx;
use widget;
use render;
use Event;

/// Object that allows you to build text widgets.
pub struct TextBuilder {
    text: String
}

impl TextBuilder {
    /// Initializes a new `TextBuilder` with default values.
    pub fn new() -> TextBuilder {
        TextBuilder {
            text: String::default()
        }
    }

    /// Requests a specific text content for the widget.
    pub fn with_text(mut self, text: &str) -> TextBuilder {
        self.text = String::from(text);
        self
    }

    /// Builds the widget.
    pub fn build_boxed<R: gfx::Resources>(self) -> Box<Text<R>> {
        Box::new(Text {
            text: self.text,
            _r: std::marker::PhantomData
        })
    }
}

/// Represents a widget with a text content.
pub struct Text<R: gfx::Resources> {
    text: String,

    _r: std::marker::PhantomData<R>
}

impl<R: gfx::Resources> widget::Widget<R> for Text<R> {
    fn raise_event(&mut self, _: &Event, _: &render::RenderArea, offset: &mut render::RenderOffset) {
        let size = [(self.text.len()*18) as i32, 18];
        offset.position[1] += size[1];

        // We don't care about events
    }

    fn render(
        &self, renderer: &mut render::Renderer<R>,
        prev_area: &render::RenderArea, offset: &mut render::RenderOffset)
    {
        // TODO: Actually get width based on the width of the result
        let pos = [
            (prev_area.position[0] + offset.position[0]),
            (prev_area.position[1] + offset.position[1])];
        let size = [(self.text.len()*18) as i32, 18];

        // Render the actual text
        renderer.render_text(pos, &self.text);

        // Increment the rendering offset for the next widget
        offset.position[1] += size[1];
    }
}
