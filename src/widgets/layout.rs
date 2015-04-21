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
use gfx::traits::*;

pub struct LayoutArea {
    pub position: [u16;2],
    pub size: [u16;2]
}

pub enum LayoutBackground {
    None,
    Color([f32;3])
}

pub struct LayoutWidgetBuilder {
    background: LayoutBackground
}

impl LayoutWidgetBuilder {
    pub fn new() -> LayoutWidgetBuilder {
        LayoutWidgetBuilder {
            background: LayoutBackground::None
        }
    }

    pub fn with_background_color(mut self, color: [f32;3]) -> LayoutWidgetBuilder {
        self.background = LayoutBackground::Color(color);
        self
    }

    pub fn build(mut self) -> LayoutWidget {
        LayoutWidget {
            background: self.background
        }
    }
}

pub struct LayoutWidget {
    background: LayoutBackground
}

impl LayoutWidget {
    pub fn set_background(&mut self, background: LayoutBackground) {
        self.background = background;
    }

    pub fn render<
        R: gfx::Resources,
        O: gfx::Output<R>,
        C: gfx::CommandBuffer<R>
    >(
        &self,
        output: &O,
        renderer: &mut gfx::Renderer<R, C>,
        render_helper: &mut ::RenderHelper<R>,
        prev_area: &LayoutArea)
    {
        self.render_background(output, renderer, render_helper, prev_area);
    }

    fn render_background<
        R: gfx::Resources,
        O: gfx::Output<R>,
        C: gfx::CommandBuffer<R>
    >(
        &self,
        output: &O,
        renderer: &mut gfx::Renderer<R, C>,
        render_helper: &mut ::RenderHelper<R>,
        area: &LayoutArea)
    {
        match self.background {
            LayoutBackground::None => {},
            LayoutBackground::Color(c) => {
                render_helper.draw_square();
            }
        }
    }
}
