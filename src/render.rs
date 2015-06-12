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
use std::cell::RefCell;
use std::collections::{HashMap};
use std::rc::Rc;
use cgmath;
use cgmath::FixedArray;
use gfx;
use gfx::traits::*;
use gfx_text;
use element::{DomElement};
use element_type::{ElementType, ElementTypes};

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

gfx_parameters!( FlatParams {
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

gfx_parameters!( TexturedParams {
    u_Transform@ transform: [[f32; 4]; 4],
    u_Texture@ texture: gfx::shade::TextureParam<R>,
});

pub struct RenderCache<R: gfx::Resources, F: gfx::Factory<R>> {
    draw_state: gfx::DrawState,
    sampler: gfx::device::handle::Sampler<R>,

    flat_program: gfx::device::handle::Program<R>,
    textured_program: gfx::device::handle::Program<R>,

    text_renderer: gfx_text::Renderer<R, F>
}

impl<R: gfx::Resources, F: gfx::Factory<R> + Clone> RenderCache<R, F> {
    pub fn new(factory: &mut F) -> Self {
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
        let text_renderer = gfx_text::new(factory.clone())
            .with_size(13)
            .with_font_data(include_bytes!("../assets/Roboto-Regular.ttf"))
            .build().unwrap();

        RenderCache {
            draw_state: state,
            sampler: sampler,

            flat_program: flat_program,
            textured_program: textured_program,

            text_renderer: text_renderer
        }
    }
}

pub trait RenderHelper<R: gfx::Resources> {
    fn render_rect_flat(&mut self, position: [i32; 2], size: [i32; 2], color: [f32; 3]);
    fn render_rect_textured(&mut self, position: [i32; 2], size: [i32; 2], texture: gfx::handle::Texture<R>);
    fn render_text(&mut self, position: [i32; 2], text: &str);
}

pub struct ConcreteRenderHelper<'a, R: gfx::Resources, S: 'a + Stream<R>, F: 'a + gfx::Factory<R>> {
    render_data: Rc<RefCell<RenderCache<R, F>>>,
    projection_matrix: [[f32; 4]; 4],

    stream: &'a mut S,
    factory: &'a mut F
}

impl<'a, R: gfx::Resources, S: Stream<R>, F: gfx::Factory<R>> ConcreteRenderHelper<'a, R, S, F> {
    pub fn new(
        stream: &'a mut S, factory: &'a mut F,
        render_data: Rc<RefCell<RenderCache<R, F>>>
    )-> Self {
        // Prepare shared uniform data that never has to change
        let (w, h) = stream.get_output().get_size();
        let proj = cgmath::ortho::<f32>(
            0.0, w as f32,
            h as f32, 0.0,
            1.0, -1.0).into_fixed();

        ConcreteRenderHelper {
            render_data: render_data,
            projection_matrix: proj,

            stream: stream,
            factory: factory
        }
    }
}

impl<'a, R: gfx::Resources, S: Stream<R>, F: gfx::Factory<R>> RenderHelper<R> for ConcreteRenderHelper<'a, R, S, F> {
    fn render_rect_flat(&mut self, position: [i32; 2], size: [i32; 2], color: [f32; 3]) {
        let render_data = &self.render_data.borrow();

        // Set up the uniform data
        let flat_params = FlatParams::<R> {
            transform: self.projection_matrix.clone(),
            _r: std::marker::PhantomData
        };

        let start = [position[0] as u16, position[1] as u16];
        let end = [position[0] as u16 + size[0] as u16, position[1] as u16 + size[1] as u16];

        // Create a mesh from the rectangle
        let mut vertices = Vec::<FlatVertex>::new();
        vertices.push(FlatVertex { pos: [ end[0], start[1] ], color: color });
        vertices.push(FlatVertex { pos: [ start[0], start[1] ], color: color });
        vertices.push(FlatVertex { pos: [ start[0], end[1] ], color: color });
        vertices.push(FlatVertex { pos: [ end[0], end[1] ], color: color });
        vertices.push(FlatVertex { pos: [ end[0], start[1] ], color: color });
        vertices.push(FlatVertex { pos: [ start[0], end[1] ], color: color });
        let mesh = self.factory.create_mesh(&vertices);
        let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);

        // Actually render that mesh
        let batch = gfx::batch::bind(&render_data.draw_state, &mesh, slice.clone(), &render_data.flat_program, &flat_params);
        self.stream.draw(&batch).unwrap();
    }

    fn render_rect_textured(&mut self, position: [i32; 2], size: [i32; 2], texture: gfx::handle::Texture<R>) {
        let render_data = &self.render_data.borrow();

        // Set up the uniform data
        let textured_params = TexturedParams::<R> {
            transform: self.projection_matrix.clone(),
            texture: (texture.clone(), Some(render_data.sampler.clone())),
            _r: std::marker::PhantomData
        };

        let start = [position[0] as u16, position[1] as u16];
        let end = [position[0] as u16 + size[0] as u16, position[1] as u16 + size[1] as u16];

        // Create a mesh from the rectangle
        let mut vertices = Vec::<TexturedVertex>::new();
        vertices.push(TexturedVertex { pos: [ end[0], start[1] ], tex_coord: [1.0, 0.0] });
        vertices.push(TexturedVertex { pos: [ start[0], start[1] ], tex_coord: [0.0, 0.0] });
        vertices.push(TexturedVertex { pos: [ start[0], end[1] ], tex_coord: [0.0, 1.0] });
        vertices.push(TexturedVertex { pos: [ end[0], end[1] ], tex_coord: [1.0, 1.0] });
        vertices.push(TexturedVertex { pos: [ end[0], start[1] ], tex_coord: [1.0, 0.0] });
        vertices.push(TexturedVertex { pos: [ start[0], end[1] ], tex_coord: [0.0, 1.0] });
        let mesh = self.factory.create_mesh(&vertices);
        let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);

        // Actually render that mesh
        let batch = gfx::batch::bind(&render_data.draw_state, &mesh, slice.clone(), &render_data.textured_program, &textured_params);
        self.stream.draw(&batch).unwrap();
    }

    fn render_text(&mut self, position: [i32; 2], text: &str) {
        let mut render_data = self.render_data.borrow_mut();
        render_data.text_renderer.add(
            text,
            position,
            [1.0, 1.0, 1.0, 1.0],
        );
        render_data.text_renderer.draw_at(self.stream, self.projection_matrix.clone()).unwrap();
    }
}

pub fn render<R: gfx::Resources, S: Stream<R>, F: gfx::Factory<R>>(
    dom: &DomElement, element_types: &mut ElementTypes<R>,
    stream: &mut S, factory: &mut F,
    render_cache: Rc<RefCell<RenderCache<R, F>>>)
{
    let mut helper = ConcreteRenderHelper::new(stream, factory, render_cache);
    render_element_recursive(dom, element_types, &mut helper);
}

fn render_element_recursive<R: gfx::Resources>(
    element: &DomElement, element_types: &mut ElementTypes<R>,
    helper: &mut RenderHelper<R>)
{
    {
        // Look up the element's type
        let element_type = element_types.get(element.tag());

        // Actually render
        element_type.render(element, helper);
    }

    // Render all the element's children
    for child in element.children() {
        render_element_recursive(child, element_types, helper);
    }
}
