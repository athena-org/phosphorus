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

pub struct TemplateElement {
    name: String,
    attrs: Vec<bool>,
    children: Vec<TemplateElement>
}

impl TemplateElement {
    pub fn new(name: &str) -> Self {
        TemplateElement {
            name: String::from(name),
            attrs: Vec::new(),
            children: Vec::new()
        }
    }

    // # Setters

    pub fn with_attr(mut self, key: &str, value: &str) -> Self {
        // TODO: Actually set attributes
        self.attrs.push(true);
        self
    }

    pub fn with_child(mut self, element: TemplateElement) -> Self {
        self.children.push(element);
        self
    }

    // # Getters

    pub fn name(&self) -> &str {
        &self.name
    }

    // # Utility

    pub fn to_dom(self) -> DomElement {
        DomElement {
            template: self,
            is_outdated: true
        }
    }
}

pub struct DomElement {
    template: TemplateElement,
    is_outdated: bool
}

impl DomElement {
    /// Initializes the DomElement tree's bindings.
    pub fn bindings_init(/* put scope table here */) {
        unimplemented!();
    }

    /// Updates the DomElement tree based on the values its elements are bound to.
    pub fn bindings_update() {
        // TODO: Actually update smartly instead of just wiping and re-creating everything

        // TODO: Wipe and re-create here
    }
}
