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

extern crate gfx;

use gfx::traits::*;

pub mod layout;

static VERTEX_SRC: &'static [u8] = b"
    #version 120

    attribute vec2 a_Pos;
    attribute vec3 a_Color;
    varying vec4 v_Color;

    void main() {
        v_Color = vec4(a_Color, 1.0);
        gl_Position = vec4(a_Pos, 0.0, 1.0);
    }
";

static FRAGMENT_SRC: &'static [u8] = b"
    #version 120

    varying vec4 v_Color;

    void main() {
        gl_FragColor = v_Color;
    }
";

struct Shaders<R: gfx::Resources> {
    solid_color_program: gfx::device::handle::Program<R>
}

pub struct Gui<R: gfx::Resources> {
    root: layout::GuiLayout,
    shaders: Shaders<R>
}

impl<R: gfx::Resources> Gui<R> {
    pub fn root(&self) -> &layout::GuiLayout { &self.root }
    pub fn root_mut(&mut self) -> &mut layout::GuiLayout { &mut self.root }

    pub fn new<F: gfx::Factory<R>>(factory: &mut F) -> Gui<R> {
        let solid_color_program = match factory.link_program(VERTEX_SRC, FRAGMENT_SRC) {
            Ok(v) => v,
            Err(e) => panic!(format!("{:?}", e))
        };

        let shaders = Shaders {
            solid_color_program: solid_color_program
        };

        Gui {
            root: layout::GuiLayout::new(),
            shaders: shaders
        }
    }

    pub fn render<
        O: gfx::Output<R>,
        C: gfx::CommandBuffer<R>
    >(  &self,
        output: &O,
        renderer: &mut gfx::Renderer<R, C>)
    {
        let (x, y) = output.get_size();
        let offset = layout::LayoutOffset {
            position: [0, 0],
            size: [x, y]
        };
        self.root.render(output, renderer, &offset);
    }
}
