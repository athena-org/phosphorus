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

#![feature(plugin, custom_attribute)]
#![plugin(gfx_macros)]

extern crate cgmath;
extern crate gfx;
extern crate gfx_text;
extern crate gfx_texture;

use gfx::traits::*;

pub mod render;
pub mod widget;

pub struct Gui<R: gfx::Resources> {
    root: widget::LayoutWidget<R>,
    render_helper: render::RenderHelper<R>
}

impl<R: gfx::Resources> Gui<R> {
    pub fn root(&self) -> &widget::LayoutWidget<R> { &self.root }
    pub fn root_mut(&mut self) -> &mut widget::LayoutWidget<R> { &mut self.root }

    pub fn new<F: gfx::Factory<R>>(factory: &mut F, root: widget::LayoutWidget<R>) -> Gui<R> {
        Gui {
            root: root,
            render_helper: render::RenderHelper::new(factory)
        }
    }

    pub fn render<
        O: gfx::Output<R>,
        C: gfx::CommandBuffer<R>,
        F: gfx::Factory<R>
    >(
        &mut self,
        output: &mut O,
        renderer: &mut gfx::Renderer<R, C>,
        factory: &mut F)
    {
        // Create our render data struct
        let mut data = render::RenderData::new();

        // Set up a layout area to the whole screen
        let (x, y) = output.get_size();
        let area = widget::RenderArea {
            position: [0, 0],
            size: [x, y]
        };

        // Actually tell the root layout to render to the data
        self.root.render(&mut data, &area);

        // Finally, render the data we've gathered
        self.render_helper.render(output, renderer, factory, data);
    }
}
