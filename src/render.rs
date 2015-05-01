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

static FLAT_VERTEX_SRC: &'static [u8] = b"
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

static FLAT_FRAGMENT_SRC: &'static [u8] = b"
    #version 150 core

    in vec4 v_Color;
    out vec4 o_Color;

    void main() {
        o_Color = v_Color;
    }
";

#[vertex_format]
#[derive(Clone, Copy)]
struct FlatVertex {
    #[name = "i_Pos"]
    pos: [u16; 2],

    #[name = "i_Color"]
    color: [f32; 3],
}

#[shader_param]
struct FlatParams<R: gfx::Resources> {
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
        let solid_color_program = match factory.link_program(FLAT_VERTEX_SRC, FLAT_FRAGMENT_SRC) {
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
        let params = FlatParams::<R> {
            transform: proj.into_fixed(),
            _dummy: std::marker::PhantomData
        };

        // Render all rectangles
        for entry in &data.entries {
            match entry {
                &RenderEntry::Flat(ref data, color) =>
                    self.render_rect_flat(output, renderer, factory, data, color, &params),
                &RenderEntry::Textured(ref data, ref texture) =>
                    self.render_rect_textured(output, renderer, factory, data, texture, &params)
            }
        }
    }

    fn render_rect_flat<O: gfx::Output<R>, C: gfx::CommandBuffer<R>, F: gfx::Factory<R>>(
        &self,
        output: &mut O, renderer: &mut gfx::Renderer<R, C>, factory: &mut F,
        rect: &Rectangle, color: [f32;3], params: &FlatParams<R>)
    {
        // Create a mesh from the rectangle
        let mut vertices = Vec::<FlatVertex>::new();
        vertices.push(FlatVertex { pos: [ rect.end[0], rect.start[1] ], color: color });
        vertices.push(FlatVertex { pos: [ rect.start[0], rect.start[1] ], color: color });
        vertices.push(FlatVertex { pos: [ rect.start[0], rect.end[1] ], color: color });
        vertices.push(FlatVertex { pos: [ rect.end[0], rect.end[1] ], color: color });
        vertices.push(FlatVertex { pos: [ rect.end[0], rect.start[1] ], color: color });
        vertices.push(FlatVertex { pos: [ rect.start[0], rect.end[1] ], color: color });
        let mesh = factory.create_mesh(&vertices);

        // Actually render that mesh
        let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);
        let batch = gfx::batch::bind(&self.draw_state, &mesh, slice.clone(), &self.solid_color_program, params);
        renderer.draw(&batch, output).unwrap();
    }

    fn render_rect_textured<O: gfx::Output<R>, C: gfx::CommandBuffer<R>, F: gfx::Factory<R>>(
        &self,
        output: &mut O, renderer: &mut gfx::Renderer<R, C>, factory: &mut F,
        rect: &Rectangle, texture: &gfx::TextureHandle<R>, params: &FlatParams<R>)
    {
        let color: [f32;3] = [1.0, 0.0, 1.0];

        // Create a mesh from the rectangle
        let mut vertices = Vec::<FlatVertex>::new();
        vertices.push(FlatVertex { pos: [ rect.end[0], rect.start[1] ], color: color });
        vertices.push(FlatVertex { pos: [ rect.start[0], rect.start[1] ], color: color });
        vertices.push(FlatVertex { pos: [ rect.start[0], rect.end[1] ], color: color });
        vertices.push(FlatVertex { pos: [ rect.end[0], rect.end[1] ], color: color });
        vertices.push(FlatVertex { pos: [ rect.end[0], rect.start[1] ], color: color });
        vertices.push(FlatVertex { pos: [ rect.start[0], rect.end[1] ], color: color });
        let mesh = factory.create_mesh(&vertices);

        // Actually render that mesh
        let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);
        let batch = gfx::batch::bind(&self.draw_state, &mesh, slice.clone(), &self.solid_color_program, params);
        renderer.draw(&batch, output).unwrap();
    }
}

pub struct Rectangle {
    start: [u16;2],
    end: [u16;2]
}

impl Rectangle {
    fn new(position: [u16;2], size: [u16;2]) -> Rectangle {
        Rectangle {
            start: [position[0], position[1]],
            end: [position[0] + size[0], position[1] + size[1]]
        }
    }
}

pub enum RenderEntry<R: gfx::Resources> {
    Flat(Rectangle, [f32;3]),
    Textured(Rectangle, gfx::TextureHandle<R>)
}

pub struct RenderData<R: gfx::Resources>
{
    entries: Vec<RenderEntry<R>>
}

impl<R: gfx::Resources> RenderData<R> {
    pub fn new() -> RenderData<R> {
        RenderData {
            entries: Vec::new()
        }
    }

    pub fn push_rect_flat(&mut self, position: [u16;2], size: [u16;2], color: [f32;3]) {
        self.entries.push(RenderEntry::Flat(Rectangle::new(position, size), color));
    }

    pub fn push_rect_textured(&mut self, position: [u16;2], size: [u16;2], texture: gfx::TextureHandle<R>) {
        self.entries.push(RenderEntry::Textured(Rectangle::new(position, size), texture));
    }
}
