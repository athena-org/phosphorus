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
use std::default::Default;
use gfx;
use widget;
use render;

pub struct TextWidgetBuilder {
    text: String
}

impl TextWidgetBuilder {
    pub fn new<'a>() -> TextWidgetBuilder {
        TextWidgetBuilder {
            text: String::default()
        }
    }

    pub fn with_text(mut self, text: &str) -> TextWidgetBuilder {
        self.text = String::from(text);
        self
    }

    pub fn build_boxed<R: gfx::Resources>(self) -> Box<TextWidget<R>> {
        Box::new(TextWidget {
            text: self.text,
            _dummy: std::marker::PhantomData
        })
    }
}

pub struct TextWidget<R: gfx::Resources> {
    text: String,

    _dummy: std::marker::PhantomData<R>
}

impl<R: gfx::Resources> widget::Widget<R> for TextWidget<R> {
    fn render(
        &self, data: &mut render::RenderData<R>,
        prev_area: &widget::RenderArea, offset: &mut widget::RenderOffset)
    {
        // TODO: Actually get width based on the width of the result
        let pos = [
            (prev_area.position[0] + offset.position[0]) as i32,
            (prev_area.position[1] + offset.position[1]) as i32];
        let size = [(self.text.len()*18) as u16, 18];

        // Render the actual text
        data.push_text(pos, self.text.clone());

        // Increment the rendering offset for the next widget
        offset.position[1] += size[1];
    }
}