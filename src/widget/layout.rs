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

use gfx;
use widget;
use render;

pub enum LayoutBackground {
    None,
    Color([f32; 3])
}

/// Object that allows you to build layout widgets.
pub struct LayoutBuilder<R: gfx::Resources> {
    background: LayoutBackground,
    widgets: Vec<Box<widget::Widget<R>>>
}

impl<R: gfx::Resources> LayoutBuilder<R> {
    /// Initializes a new `LayoutBuilder` with default values.
    pub fn new() -> LayoutBuilder<R> {
        LayoutBuilder {
            background: LayoutBackground::None,
            widgets: Vec::new()
        }
    }

    pub fn with_background_color(mut self, color: [u8; 3]) -> LayoutBuilder<R> {
        let rgb = [(color[0] as f32)/255.0, (color[1] as f32)/255.0, (color[2] as f32)/255.0];
        self.background = LayoutBackground::Color(rgb);
        self
    }

    pub fn with_widget(mut self, widget: Box<widget::Widget<R>>) -> LayoutBuilder<R> {
        self.widgets.push(widget);
        self
    }

    pub fn build(self) -> Layout<R> {
        Layout {
            background: self.background,
            widgets: self.widgets
        }
    }
}

pub struct Layout<R: gfx::Resources> {
    background: LayoutBackground,
    widgets: Vec<Box<widget::Widget<R>>>
}

impl<R: gfx::Resources> Layout<R> {
    pub fn set_background(&mut self, background: LayoutBackground) {
        self.background = background;
    }

    pub fn render(&self, renderer: &mut render::Renderer<R>, prev_area: &render::RenderArea)
    {
        self.render_background(renderer, prev_area);

        // Render all child widgets
        let mut offset = render::RenderOffset {position: [0, 0]};
        for widget in &self.widgets {
            widget.render(renderer, prev_area, &mut offset);
        }
    }

    fn render_background(
        &self,
        renderer: &mut render::Renderer<R>,
        area: &render::RenderArea)
    {
        match self.background {
            // Different background types render differently
            LayoutBackground::None => {},
            LayoutBackground::Color(c) => {
                renderer.render_rect_flat(area.position, area.size, c);
            }
        }
    }
}
