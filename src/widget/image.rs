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

use std::default::Default;
use gfx;
use gfx_texture;
use widget;
use render;

pub struct ImageBuilder {
    image_source: String,
    size: [u16;2]
}

impl Default for ImageBuilder {
    fn default() -> ImageBuilder {
        ImageBuilder {
            image_source: String::default(),
            size: [0, 0]
        }
    }
}

impl ImageBuilder {
    pub fn new() -> ImageBuilder {
        ImageBuilder::default()
    }

    pub fn with_source(mut self, source: &str) -> ImageBuilder {
        self.image_source = String::from(source);
        self
    }

    pub fn with_size(mut self, size: [u16;2]) -> ImageBuilder {
        self.size = size;
        self
    }

    pub fn build_boxed<R: gfx::Resources, F: gfx::Factory<R>>(self, factory: &mut F) -> Box<Image<R>> {
        let settings = gfx_texture::Settings::new();
        let tex = gfx_texture::Texture::from_path(factory, self.image_source, &settings).unwrap();

        Box::new(Image {
            texture: tex,
            size: self.size
        })
    }
}

pub struct Image<R: gfx::Resources> {
    texture: gfx_texture::Texture<R>,
    size: [u16;2]
}

impl<R: gfx::Resources> widget::Widget<R> for Image<R> {
    fn render(
        &self, data: &mut render::RenderData<R>,
        prev_area: &render::RenderArea, offset: &mut render::RenderOffset)
    {
        let pos = [prev_area.position[0] + offset.position[0], prev_area.position[1] + offset.position[1]];
        data.push_rect_textured(pos, self.size, self.texture.handle());

        // Increment the rendering offset for the next widget
        offset.position[1] += self.size[1];
    }
}
