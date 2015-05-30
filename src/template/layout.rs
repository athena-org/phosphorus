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

use template::{TemplateNode};

pub struct Layout {
    children: Vec<Box<TemplateNode>>
}

impl Layout {
    pub fn new() -> Self {
        Layout {
            children: Vec::new()
        }
    }

    pub fn with_child<T: TemplateNode + 'static>(mut self, node: T) -> Self {
        self.children.push(Box::new(node));
        self
    }
}

impl TemplateNode for Layout {
}
