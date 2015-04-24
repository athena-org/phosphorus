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

use gfx;
use gfx_texture;
use widgets;
use render;

pub struct ImageWidgetBuilder {
    image_path: String,
    size: [u16;2]
}

impl ImageWidgetBuilder {
    fn with_image(mut self, path: String) -> ImageWidgetBuilder {
        self.image_path = path;
        self
    }

    fn with_size(mut self, size: [u16;2]) -> ImageWidgetBuilder {
        self.size = size;
        self
    }

    fn build_boxed<R: gfx::Resources, F: gfx::Factory<R>>(self, factory: &mut F) -> Box<ImageWidget<R>> {
        let mut settings = gfx_texture::Settings::new();
        let tex = gfx_texture::Texture::from_path(factory, self.image_path, &settings).unwrap();

        Box::new(ImageWidget {
            texture: tex,
            size: self.size
        })
    }
}

pub struct ImageWidget<R: gfx::Resources> {
    texture: gfx_texture::Texture<R>,
    size: [u16;2]
}

impl<R: gfx::Resources> widgets::Widget for ImageWidget<R> {
    fn render(
        &self, data: &mut render::RenderData,
        prev_area: &widgets::RenderArea, offset: &mut widgets::RenderOffset)
    {
        offset.position[1] += self.size[1];
    }
}
