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
use element::{DomElement};
use render::{RenderHelper};

pub trait ElementType<R: gfx::Resources> {
    fn render(&mut self, element: &DomElement, helper: &mut RenderHelper<R>);
}

pub struct BlockType;

impl<R: gfx::Resources> ElementType<R> for BlockType {
    fn render(&mut self, element: &DomElement, helper: &mut RenderHelper<R>) {
        let size = element.size();

        let background_color = match element.attr_as::<Vec<i32>>("style_background") {
            Some(c) => [c[0] as f32 / 255.0, c[1] as f32 / 255.0, c[2] as f32 / 255.0],
            None => [0.0, 0.0, 0.0]
        };

        helper.render_rect_flat([0, 0], [size[0] as i32, size[1] as i32], background_color);
    }
}
