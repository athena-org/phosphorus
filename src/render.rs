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
use gfx::traits::*;
use cgmath;
use cgmath::FixedArray;

static VERTEX_SRC: &'static [u8] = b"
    #version 130

    attribute ivec2 a_Pos;
    attribute vec3 a_Color;
    varying vec4 v_Color;

    uniform mat4 u_Transform;

    void main() {
        v_Color = vec4(a_Color, 1.0);
        gl_Position = u_Transform * vec4(a_Pos, 0.0, 1.0);
    }
";

static FRAGMENT_SRC: &'static [u8] = b"
    #version 130

    varying vec4 v_Color;

    void main() {
        gl_FragColor = v_Color;
    }
";

#[vertex_format]
#[derive(Clone, Copy)]
struct Vertex {
    #[name = "a_Pos"]
    pos: [u16; 2],

    #[name = "a_Color"]
    color: [f32; 3],
}

#[shader_param]
struct Params<R: gfx::Resources> {
    #[name = "u_Transform"]
    transform: [[f32; 4]; 4],

    _dummy: std::marker::PhantomData<R>
}

pub struct RenderHelper<R: gfx::Resources> {
    solid_color_program: gfx::device::handle::Program<R>,
    draw_state: gfx::DrawState
}

impl<R: gfx::Resources> RenderHelper<R> {
    pub fn new<F: gfx::Factory<R>>(factory: &mut F) -> RenderHelper<R> {
        // Set up the stuff we'll need to render
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
        // Prepare the uniforms to be used for rendering
        let (x, y) = output.get_size();
        let proj = cgmath::ortho::<f32>(0.0, x as f32, y as f32, 0.0, 1.0, -1.0);
        let params = Params::<R> {
            transform: proj.into_fixed(),
            _dummy: std::marker::PhantomData
        };

        // Render all rectangles
        let rects_mesh = RenderHelper::<R>::create_rects_mesh(factory, data.rectangles);
        self.render_rects_mesh(output, renderer, &params, rects_mesh);
    }

    fn create_rects_mesh<F: gfx::Factory<R>>(factory: &mut F, rects: Vec<RectangleData>) -> gfx::Mesh<R> {
        let mut vertices = Vec::<Vertex>::new();

        for rect in rects {
            // Add our rectangle's vertices
            vertices.push(Vertex { pos: [ rect.end[0], rect.start[1] ], color: rect.color });
            vertices.push(Vertex { pos: [ rect.start[0], rect.start[1] ], color: rect.color });
            vertices.push(Vertex { pos: [ rect.start[0], rect.end[1] ], color: rect.color });

            vertices.push(Vertex { pos: [ rect.end[0], rect.end[1] ], color: rect.color });
            vertices.push(Vertex { pos: [ rect.end[0], rect.start[1] ], color: rect.color });
            vertices.push(Vertex { pos: [ rect.start[0], rect.end[1] ], color: rect.color });
        }

        // Turn the vertices into a mesh
        factory.create_mesh(&vertices)
    }

    fn render_rects_mesh<O: gfx::Output<R>, C: gfx::CommandBuffer<R>>(
        &self,
        output: &mut O, renderer: &mut gfx::Renderer<R, C>,
        params: &Params<R>, mesh: gfx::Mesh<R>)
    {
        let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);
        let batch = gfx::batch::bind(&self.draw_state, &mesh, slice.clone(), &self.solid_color_program, params);
        renderer.draw(&batch, output).unwrap();
    }
}

pub struct RenderData {
    rectangles: Vec<RectangleData>
}

pub struct RectangleData {
    start: [u16;2],
    end: [u16;2],
    color: [f32;3]
}

impl RenderData {
    pub fn new() -> RenderData {
        RenderData {
            rectangles: Vec::<RectangleData>::new()
        }
    }

    pub fn push_rectangle(&mut self, position: [u16;2], size: [u16;2], color: [f32;3]) {
        let data = RectangleData {
            start: [position[0], position[1]],
            end: [position[0] + size[0], position[1] + size[1]],
            color: color
        };

        self.rectangles.push(data);
    }
}
