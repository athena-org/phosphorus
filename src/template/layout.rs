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

use std::rc::{Rc};
use runtime;
use runtime::{RuntimeNode};
use template::{TemplateNode};

#[derive(Clone)]
pub enum LayoutBackground {
    None,
    Color([f32; 3])
}

#[derive(Clone)]
pub struct Layout {
    children: Vec<Box<TemplateNode>>,
    background: LayoutBackground
}

impl Layout {
    pub fn new() -> Self {
        Layout {
            children: Vec::new(),
            background: LayoutBackground::None
        }
    }

    pub fn with_child<T: TemplateNode + 'static>(mut self, node: T) -> Self {
        self.children.push(Box::new(node));
        self
    }

    pub fn children(&self) -> Vec<Box<TemplateNode>> {
        self.children.clone()
    }

    pub fn with_background_color(mut self, color: [u8; 3]) -> Self {
        let rgb = [(color[0] as f32)/255.0, (color[1] as f32)/255.0, (color[2] as f32)/255.0];
        self.background = LayoutBackground::Color(rgb);
        self
    }

    pub fn background(&self) -> LayoutBackground {
        self.background.clone()
    }
}

impl TemplateNode for Layout {
    fn create_runtime(self) -> Box<RuntimeNode> {
        unimplemented!();
    }

    fn clone_boxed(&self) -> Box<TemplateNode> {
        Box::new(self.clone())
    }
}
