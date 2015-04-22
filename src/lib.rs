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

extern crate gfx;

use gfx::traits::*;

pub mod widgets;

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

#[vertex_format]
#[derive(Clone, Copy)]
struct Vertex {
    #[name = "a_Pos"]
    pos: [f32; 2],

    #[name = "a_Color"]
    color: [f32; 3],
}

pub struct RenderHelper<R: gfx::Resources> {
    solid_color_program: gfx::device::handle::Program<R>,
    draw_state: gfx::DrawState
}

impl<R: gfx::Resources> RenderHelper<R> {
    pub fn draw_square<O: gfx::Output<R>, C: gfx::CommandBuffer<R>, F: gfx::Factory<R>>(&mut self,
        output: &mut O, renderer: &mut gfx::Renderer<R, C>, factory: &mut F)
    {
        let vertex_data = [
            Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
            Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
            Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] },
        ];
        let test_mesh = factory.create_mesh(&vertex_data);
        let slice = test_mesh.to_slice(gfx::PrimitiveType::TriangleList);
        let data = None;
        let batch = gfx::batch::bind(&self.draw_state, &test_mesh, slice.clone(), &self.solid_color_program, &data);
        renderer.draw(&batch, output);
    }
}

pub struct Gui<R: gfx::Resources> {
    root: widgets::LayoutWidget,
    render_helper: RenderHelper<R>
}

impl<R: gfx::Resources> Gui<R> {
    pub fn root(&self) -> &widgets::LayoutWidget { &self.root }
    pub fn root_mut(&mut self) -> &mut widgets::LayoutWidget { &mut self.root }

    pub fn new<F: gfx::Factory<R>>(factory: &mut F, root: widgets::LayoutWidget) -> Gui<R> {
        let solid_color_program = match factory.link_program(VERTEX_SRC, FRAGMENT_SRC) {
            Ok(v) => v,
            Err(e) => panic!(format!("{:?}", e))
        };

        let state = gfx::DrawState::new();
        let render_helper = RenderHelper {
            solid_color_program: solid_color_program,
            draw_state: state
        };

        Gui {
            root: root,
            render_helper: render_helper
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
        let (x, y) = output.get_size();
        let area = widgets::LayoutArea {
            position: [0, 0],
            size: [x, y]
        };
        self.root.render(output, renderer, &mut self.render_helper, factory, &area);
    }
}
