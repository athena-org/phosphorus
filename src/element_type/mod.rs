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

use std::collections::{HashMap};
use gfx;
use element::{DomElement};
use render::{RenderHelper};

pub trait ElementType<R: gfx::Resources> {
    fn render(&mut self, element: &DomElement, position: [i32; 2], helper: &mut RenderHelper<R>);
}


pub struct ElementTypes<R: gfx::Resources> {
    default: Box<ElementType<R>>,
    types: HashMap<String, Box<ElementType<R>>>
}

impl<R: gfx::Resources> ElementTypes<R> {
    pub fn new(default: Box<ElementType<R>>) -> Self {
        ElementTypes {
            default: default,
            types: HashMap::new()
        }
    }

    pub fn get(&mut self, key: &str) -> &mut Box<ElementType<R> + 'static> {
        match self.types.get_mut(key) {
            Some(t) => t,
            None => &mut self.default
        }
    }

    pub fn register(&mut self, key: &str, element_type: Box<ElementType<R>>) {
        self.types.insert(String::from(key), element_type);
    }
}


pub struct BlockType;

impl<R: gfx::Resources> ElementType<R> for BlockType {
    fn render(&mut self, element: &DomElement, position: [i32; 2], helper: &mut RenderHelper<R>) {
        let size = element.attr_as::<Vec<i32>>("style_size")
            .and_then(|v| if v.len() == 2 { Some(v) } else { None })
            .map(|v| [v[0], v[1]])
            .unwrap_or([100, 100]);

        let background_color_o = element.attr_as::<Vec<i32>>("style_background")
            .and_then(|v| if v.len() == 3 { Some(v) } else { None })
            .map(|v| [v[0] as f32 / 255.0, v[1] as f32 / 255.0, v[2] as f32 / 255.0]);

        // If we have a background color, render the background, if not just ignore it
        if let Some(background_color) = background_color_o {
            helper.render_rect_flat(position, size, background_color);
        }
    }
}

pub struct TextType;

impl<R: gfx::Resources> ElementType<R> for TextType {
    fn render(&mut self, element: &DomElement, position: [i32; 2], helper: &mut RenderHelper<R>) {
        // Render the base block type as background
        BlockType.render(element, position, helper);

        // If we have text, render it over it
        if let Some(text) = element.attr("value") {
            helper.render_text(position, &text);
        }
    }
}
