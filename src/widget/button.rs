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
use Event;

/// Object that allows you to build button widgets.
pub struct ButtonBuilder {
    text: String,
    size: [i32; 2],
    callback: Option<Box<Fn()>>
}

impl ButtonBuilder {
    /// Initializes a new `ButtonBuilder` with default values.
    pub fn new() -> ButtonBuilder {
        ButtonBuilder {
            text: String::default(),
            size: [80, 20],
            callback: None
        }
    }

    /// Requests a text content for the widget.
    pub fn with_text(mut self, text: &str) -> ButtonBuilder {
        self.text = String::from(text);
        self
    }

    // Requests a callback to be called on click.
    pub fn with_callback(mut self, callback: Box<Fn()>) -> ButtonBuilder {
        self.callback = Some(callback);
        self
    }

    /// Builds the widget.
    pub fn build_boxed<R: gfx::Resources>(self) -> Box<Button<R>> {
        Box::new(Button {
            text: self.text,
            size: self.size,
            callback: self.callback,

            hovering: false,

            _r: std::marker::PhantomData
        })
    }
}

/// Represents a widget that detects mouse click input.
pub struct Button<R: gfx::Resources> {
    text: String,
    size: [i32; 2],
    callback: Option<Box<Fn()>>,

    hovering: bool,

    _r: std::marker::PhantomData<R>
}

impl<R: gfx::Resources> widget::Widget<R> for Button<R> {
    fn raise_event(
        &mut self, event: &Event,
        prev_area: &render::RenderArea, offset: &mut render::RenderOffset)
    {
        match event {
            &Event::MouseMoved(mouse_pos) => {
                let pos = [
                    (prev_area.position[0] + offset.position[0]),
                    (prev_area.position[1] + offset.position[1])];

                self.hovering =
                    mouse_pos[0] > pos[0] && mouse_pos[1] > pos[1] &&
                    mouse_pos[0] <= pos[0] + self.size[0] && mouse_pos[1] <= pos[1] + self.size[1];
            },
            &Event::MouseClick => {
                if self.hovering {
                    if let &Some(ref c) = &self.callback {
                        c();
                    }
                }
            },
            _ => {}
        }

        // Increment the rendering offset for the next widget
        offset.position[1] += self.size[1];
    }

    fn render(
        &self, renderer: &mut render::Renderer<R>,
        prev_area: &render::RenderArea, offset: &mut render::RenderOffset)
    {
        let pos = [
            (prev_area.position[0] + offset.position[0]),
            (prev_area.position[1] + offset.position[1])];

        renderer.render_rect_flat(pos, self.size, if self.hovering {[0.34, 0.34, 0.34]} else {[0.28, 0.28, 0.28]});
        renderer.render_text([pos[0] + 4, pos[1] + 1], &self.text);

        // Increment the rendering offset for the next widget
        offset.position[1] += self.size[1];
    }
}
