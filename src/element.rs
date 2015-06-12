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

#[derive(Clone)]
pub struct TemplateElement {
    tag: String,
    attrs: HashMap<String, String>,
    children: Vec<TemplateElement>
}

impl TemplateElement {
    pub fn new(tag: &str) -> Self {
        TemplateElement {
            tag: String::from(tag),
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

    pub fn tag(&self) -> &str {
        &self.tag
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
            children: Vec::new(),
            is_outdated: true
        }
    }
}

pub struct DomElement {
    template: TemplateElement,
    children: Vec<DomElement>,
    is_outdated: bool
}

impl DomElement {
    /// Updates all outdated information with new information if needed.
    pub fn update_outdated(&mut self) {
        if !self.is_outdated { return; }

        self.children.clear();

        for child in self.template.children() {
            self.children.push(child.clone().to_dom());
        }

        self.is_outdated = false;
    }

    pub fn tag(&self) -> &str {
        self.template.tag()
    }

    pub fn attr(&self, key: &str) -> Option<String> {
        self.template.attrs().get(key).map(|s| s.clone())
    }

    pub fn children(&self) -> &Vec<DomElement> {
        &self.children
    }
}

mod domelement_attr_utils {
    use rustc_serialize::json;
        use super::{DomElement};

    impl DomElement {
        pub fn attr_as<T: ::rustc_serialize::Decodable>(&self, key: &str) -> Option<T> {
            let val_str = match self.attr(key) {
                Some(s) => s,
                None => return None
            };

            // TODO: Change this to use serde instead of rustc_serialize
            match json::decode::<T>(&val_str) {
                Ok(v) => Some(v),
                Err(e) => None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use element::{TemplateElement};

    #[test]
    fn domelement_attr_looks_up_value() {
        // Arrange
        let element = TemplateElement::new("test")
            .with_attr("foo", "bar")
            .to_dom();

        // Act
        let bar = element.attr("foo");

        // Assert
        assert!(bar.is_some());
        assert!(bar.unwrap() == "bar");
    }

    #[test]
    fn domelement_attr_as_deserializes_value() {
        // Arrange
        let element = TemplateElement::new("test")
            .with_attr("style_size", "[500, 92]")
            .to_dom();

        // Act
        let o_size = element.attr_as::<Vec<i32>>("style_size");

        // Assert
        assert!(o_size.is_some());
        let size = o_size.unwrap();
        assert!(size.len() == 2);
        assert!(size[0] == 500);
        assert!(size[1] == 92);
    }

    #[test]
    fn domelement_attr_as_does_not_panic_on_invalid_or_lacking_data() {
        // Arrange
        let element = TemplateElement::new("test")
            .with_attr("invalid", "akfiajc83C$)YN&#0")
            .to_dom();

        // Act
        let lac = element.attr_as::<Vec<i32>>("lacking");
        let inv = element.attr_as::<Vec<i32>>("invalid");

        // Assert
        assert!(lac.is_none());
        assert!(inv.is_none());
    }
}
