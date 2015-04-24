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

use widgets;
use render;

pub struct TextWidgetBuilder {
    text: String
}

impl TextWidgetBuilder {
    pub fn new() -> TextWidgetBuilder {
        TextWidgetBuilder {
            text: String::new()
        }
    }

    pub fn with_text(mut self, text: &str) -> TextWidgetBuilder {
        self.text = String::from(text);
        self
    }

    pub fn build_boxed(self) -> Box<TextWidget> {
        Box::new(TextWidget {
            text: self.text
        })
    }
}

pub struct TextWidget {
    text: String
}

impl widgets::Widget for TextWidget {
    fn render(
        &self, data: &mut render::RenderData,
        prev_area: &widgets::RenderArea, offset: &mut widgets::RenderOffset)
    {
        let pos = [prev_area.position[0] + offset.position[0], prev_area.position[1] + offset.position[1]];
        let size = [(self.text.len()*20) as u16, 20];
        data.push_rectangle(pos, size, [0.0, 1.0, 1.0]);

        // Increment the rendering offset for the next widget
        offset.position[1] += size[1];

        // Quick notes for implementing showing text:
        // https://github.com/PistonDevelopers/gfx_texture
        // https://github.com/kvark/claymore/blob/master/src/load/lib.rs#L114-L119
        // https://github.com/retep998/winapi-rs/blob/master/lib/user32-sys/src/lib.rs#L351
        // http://stackoverflow.com/questions/14762456/getclipboarddatacf-text
    }
}
