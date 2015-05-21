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
use Event;

/// Object that allows you to build image widgets.
pub struct ImageBuilder {
    image_source: String,
    size: [i32; 2]
}

impl ImageBuilder {
    /// Initializes a new `ImageBuilder` with default values.
    pub fn new() -> ImageBuilder {
        ImageBuilder {
            image_source: String::default(),
            size: [0, 0]
        }
    }

    /// Requests an image to be loaded in to be displayed.
    pub fn with_source(mut self, source: &str) -> ImageBuilder {
        self.image_source = String::from(source);
        self
    }

    /// Requests a size to show the widget at.
    pub fn with_size(mut self, size: [i32; 2]) -> ImageBuilder {
        self.size = size;
        self
    }

    /// Builds the widget.
    pub fn build_boxed<R: gfx::Resources, F: gfx::Factory<R>>(self, factory: &mut F) -> Box<Image<R>> {
        let settings = gfx_texture::Settings::new();
        let tex = gfx_texture::Texture::from_path(factory, self.image_source, &settings).unwrap();

        Box::new(Image {
            texture: tex,
            size: self.size
        })
    }
}

/// Represents a widget with an image content.
pub struct Image<R: gfx::Resources> {
    texture: gfx_texture::Texture<R>,
    size: [i32;2]
}

impl<R: gfx::Resources> widget::Widget<R> for Image<R> {
    fn raise_event(&mut self, _: &Event, _: &render::RenderArea, offset: &mut render::RenderOffset) {
        offset.position[1] += self.size[1];

        // We don't care about events
    }

    fn render(
        &self, renderer: &mut render::Renderer<R>,
        prev_area: &render::RenderArea, offset: &mut render::RenderOffset)
    {
        let pos = [prev_area.position[0] + offset.position[0], prev_area.position[1] + offset.position[1]];
        renderer.render_rect_textured(pos, self.size, self.texture.handle());

        // Increment the rendering offset for the next widget
        offset.position[1] += self.size[1];
    }
}
