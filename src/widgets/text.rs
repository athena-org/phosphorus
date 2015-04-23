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
}
