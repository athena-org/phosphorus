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
use widgets;
use render;

pub enum LayoutBackground {
    None,
    Color([f32;3])
}

pub struct LayoutWidgetBuilder<R: gfx::Resources> {
    background: LayoutBackground,
    widgets: Vec<Box<widgets::Widget<R>>>
}

impl<R: gfx::Resources> LayoutWidgetBuilder<R> {
    pub fn new() -> LayoutWidgetBuilder<R> {
        LayoutWidgetBuilder {
            background: LayoutBackground::None,
            widgets: Vec::new()
        }
    }

    pub fn with_background_color(mut self, color: [f32;3]) -> LayoutWidgetBuilder<R> {
        self.background = LayoutBackground::Color(color);
        self
    }

    pub fn with_widget(mut self, widget: Box<widgets::Widget<R>>) -> LayoutWidgetBuilder<R> {
        self.widgets.push(widget);
        self
    }

    pub fn build(self) -> LayoutWidget<R> {
        LayoutWidget {
            background: self.background,
            widgets: self.widgets
        }
    }
}

pub struct LayoutWidget<R: gfx::Resources> {
    background: LayoutBackground,
    widgets: Vec<Box<widgets::Widget<R>>>
}

impl<R: gfx::Resources> LayoutWidget<R> {
    pub fn set_background(&mut self, background: LayoutBackground) {
        self.background = background;
    }

    pub fn render(&self, data: &mut render::RenderData<R>, prev_area: &widgets::RenderArea)
    {
        self.render_background(data, prev_area);

        // Render all child widgets
        let mut offset = widgets::RenderOffset {position: [0, 0]};
        for widget in &self.widgets {
            widget.render(data, prev_area, &mut offset);
        }
    }

    fn render_background(
        &self,
        data: &mut render::RenderData<R>,
        area: &widgets::RenderArea)
    {
        match self.background {
            // Different background types render differently
            LayoutBackground::None => {},
            LayoutBackground::Color(c) => {
                data.push_rect_flat(area.position, area.size, c);
            }
        }
    }
}
