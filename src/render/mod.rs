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
use cgmath;
use cgmath::FixedArray;
use gfx;
use gfx::traits::*;
use gfx_text;
use widget;

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

gfx_vertex!( FlatVertex {
    i_Pos@ pos: [u16; 2],
    i_Color@ color: [f32; 3],
});

gfx_parameters!( FlatParams/FlatParamsLink {
    u_Transform@ transform: [[f32; 4]; 4],
});

static TEXTURED_VERTEX_SRC: &'static [u8] = b"
    #version 150 core

    in ivec2 i_Pos;
    in vec2 i_TexCoord;
    out vec2 v_TexCoord;

    uniform mat4 u_Transform;

    void main() {
        v_TexCoord = i_TexCoord;
        gl_Position = u_Transform * vec4(i_Pos, 0.0, 1.0);
    }
";

static TEXTURED_FRAGMENT_SRC: &'static [u8] = b"
    #version 150 core

    in vec2 v_TexCoord;
    out vec4 o_Color;

    uniform sampler2D u_Texture;

    void main() {
        o_Color = texture(u_Texture, v_TexCoord);
    }
";

gfx_vertex!( TexturedVertex {
    i_Pos@ pos: [u16; 2],
    i_TexCoord@ tex_coord: [f32; 2],
});

gfx_parameters!( TexturedParams/TexturedParamsLink {
    u_Transform@ transform: [[f32; 4]; 4],
    u_Texture@ texture: gfx::shade::TextureParam<R>,
});

pub struct RenderHelper<R: gfx::Resources> {
    flat_program: gfx::device::handle::Program<R>,
    textured_program: gfx::device::handle::Program<R>,
    draw_state: gfx::DrawState,
    sampler: gfx::device::handle::Sampler<R>,
    text_renderer: gfx_text::Renderer<R>
}

impl<R: gfx::Resources> RenderHelper<R> {
    pub fn new<F: gfx::Factory<R>>(factory: &mut F) -> RenderHelper<R> {
        // Set up the stuff we'll need to render
        let flat_program = match factory.link_program(FLAT_VERTEX_SRC, FLAT_FRAGMENT_SRC) {
            Ok(v) => v,
            Err(e) => panic!(format!("{:?}", e))
        };
        let textured_program = match factory.link_program(TEXTURED_VERTEX_SRC, TEXTURED_FRAGMENT_SRC) {
            Ok(v) => v,
            Err(e) => panic!(format!("{:?}", e))
        };

        let state = gfx::DrawState::new();
        let sampler = factory.create_sampler(
            gfx::tex::SamplerInfo::new(
                gfx::tex::FilterMethod::Bilinear,
                gfx::tex::WrapMode::Clamp));

        // Set up our text renderer
        let text_renderer = gfx_text::new(factory)
            .with_size(13)
            .with_font("examples/assets/Roboto-Regular.ttf")
            .build().unwrap();

        RenderHelper {
            flat_program: flat_program,
            textured_program: textured_program,
            draw_state: state,
            sampler: sampler,
            text_renderer: text_renderer
        }
    }

    pub fn render<F: gfx::Factory<R>, S: Stream<R>>(
        &mut self,
        factory: &mut F, stream: &mut S,
        data: RenderData<R>,
        area: &widget::RenderArea)
    {
        // Prepare shared uniform data that never has to change
        let proj = cgmath::ortho::<f32>(0.0, area.size[0] as f32, area.size[1] as f32, 0.0, 1.0, -1.0).into_fixed();
        let flat_params = FlatParams::<R> {
            transform: proj.clone(),
            _r: std::marker::PhantomData
        };

        // Render all rectangles
        // TODO: This requires some serious optimization but it will work for now
        for entry in &data.entries {
            match entry {
                &RenderEntry::Flat(ref rectangle, color) =>
                    self.render_rect_flat(factory, stream, rectangle, color, &flat_params),
                &RenderEntry::Textured(ref rectangle, ref texture) =>
                    self.render_rect_textured(factory, stream, rectangle, texture, &proj),
                &RenderEntry::Text(ref position, ref string) => {
                    self.render_text(factory, stream, *position, string, &proj);
                }
            }
        }
    }

    fn render_rect_flat<F: gfx::Factory<R>, S: Stream<R>>(
        &self,
        factory: &mut F, stream: &mut S,
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
        let batch = gfx::batch::bind(&self.draw_state, &mesh, slice.clone(), &self.flat_program, params);
        stream.draw(&batch).unwrap();
    }

    fn render_rect_textured<F: gfx::Factory<R>, S: Stream<R>>(
        &self,
        factory: &mut F, stream: &mut S,
        rect: &Rectangle, texture: &gfx::handle::Texture<R>, proj: &[[f32;4];4])
    {
        let textured_params = TexturedParams::<R> {
            transform: proj.clone(),
            texture: (texture.clone(), Some(self.sampler.clone())),
            _r: std::marker::PhantomData
        };

        // Create a mesh from the rectangle
        let mut vertices = Vec::<TexturedVertex>::new();
        vertices.push(TexturedVertex { pos: [ rect.end[0], rect.start[1] ], tex_coord: [1.0, 0.0] });
        vertices.push(TexturedVertex { pos: [ rect.start[0], rect.start[1] ], tex_coord: [0.0, 0.0] });
        vertices.push(TexturedVertex { pos: [ rect.start[0], rect.end[1] ], tex_coord: [0.0, 1.0] });
        vertices.push(TexturedVertex { pos: [ rect.end[0], rect.end[1] ], tex_coord: [1.0, 1.0] });
        vertices.push(TexturedVertex { pos: [ rect.end[0], rect.start[1] ], tex_coord: [1.0, 0.0] });
        vertices.push(TexturedVertex { pos: [ rect.start[0], rect.end[1] ], tex_coord: [0.0, 1.0] });
        let mesh = factory.create_mesh(&vertices);

        // Actually render that mesh
        let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);
        let batch = gfx::batch::bind(&self.draw_state, &mesh, slice.clone(), &self.textured_program, &textured_params);
        stream.draw(&batch).unwrap();
    }

    fn render_text<F: gfx::Factory<R>, S: Stream<R>>(
        &mut self,
        factory: &mut F, stream: &mut S,
        position: [i32; 2], text: &String,
        proj: &[[f32; 4]; 4])
    {
        self.text_renderer.draw(
            text,
            position,
            [1.0, 1.0, 1.0, 1.0],
        );
        self.text_renderer.draw_end_at(factory, stream, proj.clone()).unwrap();
    }
}

#[derive(Debug)]
struct Rectangle {
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

enum RenderEntry<R: gfx::Resources> {
    Flat(Rectangle, [f32;3]),
    Textured(Rectangle, gfx::handle::Texture<R>),
    Text([i32; 2], String)
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

    pub fn push_rect_flat(&mut self, position: [u16; 2], size: [u16; 2], color: [f32; 3]) {
        self.entries.push(RenderEntry::Flat(Rectangle::new(position, size), color));
    }

    pub fn push_rect_textured(&mut self, position: [u16; 2], size: [u16; 2], texture: gfx::handle::Texture<R>) {
        self.entries.push(RenderEntry::Textured(Rectangle::new(position, size), texture));
    }

    pub fn push_text(&mut self, position: [i32; 2], text: String) {
        self.entries.push(RenderEntry::Text(position, text));
    }
}
