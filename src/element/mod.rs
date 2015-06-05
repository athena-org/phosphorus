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
use rustc_serialize::json;

pub struct TemplateElement {
    name: String,
    attrs: HashMap<String, String>,
    children: Vec<TemplateElement>
}

impl TemplateElement {
    pub fn new(name: &str) -> Self {
        TemplateElement {
            name: String::from(name),
            attrs: HashMap::new(),
            children: Vec::new()
        }
    }

    // # Setters

    pub fn with_attr(mut self, key: &str, value: &str) -> Self {
        self.attrs.insert(String::from(key), String::from(value));
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

    pub fn attrs(&self) -> &HashMap<String, String> {
        &self.attrs
    }

    pub fn children(&self) -> &Vec<TemplateElement> {
        &self.children
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

    pub fn get_attr(&self, key: &str) -> Option<String> {
        self.template.attrs().get(key).map(|s| s.clone())
    }

    pub fn get_attr_as<T: ::rustc_serialize::Decodable>(&self, key: &str) -> Option<T> {
        let val_str = match self.get_attr(key) {
            Some(s) => s,
            None => return None
        };

        // TODO: Change this to use serde instead of rustc_serialize
        match json::decode::<T>(&val_str) {
            Ok(v) => Some(v),
            Err(e) => None
        }
    }

    pub fn get_size(&self) -> [f32; 2] {
        let default = [100.0, 100.0];

        let size = match self.get_attr_as::<Vec<f32>>("style_size") {
            Some(s) => s,
            None => return default
        };

        // Turn the vector into an array
        if size.len() != 2 { return default; }
        [*size.get(0).unwrap(), *size.get(1).unwrap()]
    }
}

#[cfg(test)]
mod tests {
    use element::{TemplateElement};

    #[test]
    fn domelement_get_attr_looks_up_value() {
        // Arrange
        let element = TemplateElement::new("test")
            .with_attr("foo", "bar")
            .to_dom();

        // Act
        let bar = element.get_attr("foo");

        // Assert
        assert!(bar.is_some());
        assert!(bar.unwrap() == "bar");
    }
}
