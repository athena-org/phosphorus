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
use gfx_texture;
use cgmath;
use cgmath::FixedArray;

static COLORED_VERTEX_SRC: &'static [u8] = b"
    #version 150 core

    in ivec2 i_Pos;
    in vec3 i_Color;
    out vec4 v_Color;

    uniform mat4 u_Transform;

    void main() {
        v_Color = vec4(i_Color, 1.0);
        gl_Position = u_Transform * vec4(i_Pos, 0.0, 1.0);
    }
";

static COLORED_FRAGMENT_SRC: &'static [u8] = b"
    #version 150 core

    in vec4 v_Color;
    out vec4 o_Color;

    void main() {
        o_Color = v_Color;
    }
";

#[vertex_format]
#[derive(Clone, Copy)]
struct ColoredVertex {
    #[name = "i_Pos"]
    pos: [u16; 2],

    #[name = "i_Color"]
    color: [f32; 3],
}

#[shader_param]
struct ColoredParams<R: gfx::Resources> {
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
        let solid_color_program = match factory.link_program(COLORED_VERTEX_SRC, COLORED_FRAGMENT_SRC) {
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
        output: &mut O, renderer: &mut gfx::Renderer<R, C>, factory: &mut F,
        data: RenderData<R>)
    {
        // Prepare the uniforms to be used for rendering
        let (x, y) = output.get_size();
        let proj = cgmath::ortho::<f32>(0.0, x as f32, y as f32, 0.0, 1.0, -1.0);
        let params = ColoredParams::<R> {
            transform: proj.into_fixed(),
            _dummy: std::marker::PhantomData
        };

        // Render all rectangles
        for rect in &data.rectangles {
            self.render_rect(output, renderer, factory, rect, &params);
        }
    }

    fn render_rect<O: gfx::Output<R>, C: gfx::CommandBuffer<R>, F: gfx::Factory<R>>(
        &self,
        output: &mut O, renderer: &mut gfx::Renderer<R, C>, factory: &mut F,
        rect: &RectangleData, params: &ColoredParams<R>)
    {
        // Create a mesh from the rectangle
        let mut vertices = Vec::<ColoredVertex>::new();
        vertices.push(ColoredVertex { pos: [ rect.end[0], rect.start[1] ], color: rect.color });
        vertices.push(ColoredVertex { pos: [ rect.start[0], rect.start[1] ], color: rect.color });
        vertices.push(ColoredVertex { pos: [ rect.start[0], rect.end[1] ], color: rect.color });
        vertices.push(ColoredVertex { pos: [ rect.end[0], rect.end[1] ], color: rect.color });
        vertices.push(ColoredVertex { pos: [ rect.end[0], rect.start[1] ], color: rect.color });
        vertices.push(ColoredVertex { pos: [ rect.start[0], rect.end[1] ], color: rect.color });
        let mesh = factory.create_mesh(&vertices);

        // Actually render that mesh
        let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);
        let batch = gfx::batch::bind(&self.draw_state, &mesh, slice.clone(), &self.solid_color_program, params);
        renderer.draw(&batch, output).unwrap();
    }
}

pub struct RenderData<'a, R: gfx::Resources> where
    R: 'a,
    R::Buffer: 'a,
    R::ArrayBuffer: 'a,
    R::Shader: 'a,
    R::Program: 'a,
    R::FrameBuffer: 'a,
    R::Surface: 'a,
    R::Texture: 'a,
    R::Sampler: 'a
{
    rectangles: Vec<RectangleData>,
    textures: Vec<&'a gfx_texture::Texture<R>>
}

pub struct RectangleData {
    start: [u16;2],
    end: [u16;2],
    color: [f32;3]
}

impl<'a, R: gfx::Resources> RenderData<'a, R> {
    pub fn new() -> RenderData<'a, R> {
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

    pub fn push_texture(&mut self, position: [u16;2], size: [u16;2], texture: &gfx_texture::Texture<R>) {
    }
}
