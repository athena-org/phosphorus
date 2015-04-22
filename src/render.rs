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
    pub fn new<F: gfx::Factory<R>>(factory: &mut F) -> RenderHelper<R> {
        let solid_color_program = match factory.link_program(VERTEX_SRC, FRAGMENT_SRC) {
            Ok(v) => v,
            Err(e) => panic!(format!("{:?}", e))
        };
        let state = gfx::DrawState::new();

        RenderHelper {
            solid_color_program: solid_color_program,
            draw_state: state
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
        factory: &mut F,
        data: RenderData)
    {
        let rects_mesh = RenderHelper::<R>::create_rects_mesh(factory, data.rectangles);
        self.render_rects_mesh(output, renderer, rects_mesh);
    }

    fn create_rects_mesh<F: gfx::Factory<R>>(factory: &mut F, rects: Vec<RectangleData>) -> gfx::Mesh<R> {
        let mut vertices = Vec::<Vertex>::new();

        for rect in rects {
            // Add our rectangle's vertices
            vertices.push(Vertex { pos: [ rect.start[0], rect.start[1] ], color: rect.color });
            vertices.push(Vertex { pos: [ rect.end[0], rect.start[1] ], color: rect.color });
            vertices.push(Vertex { pos: [ rect.start[0], rect.end[1] ], color: rect.color });

            vertices.push(Vertex { pos: [ rect.end[0], rect.start[1] ], color: rect.color });
            vertices.push(Vertex { pos: [ rect.end[0], rect.end[1] ], color: rect.color });
            vertices.push(Vertex { pos: [ rect.start[0], rect.end[1] ], color: rect.color });
        }

        // Turn the vertices into a mesh
        factory.create_mesh(&vertices)
    }

    fn render_rects_mesh<O: gfx::Output<R>, C: gfx::CommandBuffer<R>>(
        &self,
        output: &mut O, renderer: &mut gfx::Renderer<R, C>,
        mesh: gfx::Mesh<R>)
    {
        let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);
        let data = None;
        let batch = gfx::batch::bind(&self.draw_state, &mesh, slice.clone(), &self.solid_color_program, &data);
        renderer.draw(&batch, output).unwrap();
    }
}

pub struct RenderData {
    rectangles: Vec<RectangleData>
}

pub struct RectangleData {
    start: [f32;2],
    end: [f32;2],
    color: [f32;3]
}

impl RenderData {
    pub fn new() -> RenderData {
        RenderData {
            rectangles: Vec::<RectangleData>::new()
        }
    }

    pub fn push_rectangle(&mut self, color: [f32;3]) {
        let data = RectangleData {
            start: [-0.5, -0.5],
            end: [0.5, 0.5],
            color: color
        };

        self.rectangles.push(data);
    }
}
