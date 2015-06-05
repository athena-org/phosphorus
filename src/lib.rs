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

//! Phosphorus is a user interface library written in rust. It is written for use in Athena, but can also be used in [Piston](https://github.com/PistonDevelopers/piston) or standalone. It can be integrated into any application that uses [gfx-rs](https://github.com/gfx-rs/gfx-rs).
//!
//! To use Phosphorus you need to create a `Gui` object, filling it with a layout.
//!
//! ```
//! let root = phosphorus::widget::LayoutBuilder::new()
//!     .with_background_color([21, 23, 24])
//!     .build();
//! let mut gui = phosphorus::Gui::new(&mut factory, root);
//! ```
//!
//! Then you can render it using a gfx `Factory` and `Stream` combination.
//!
//! ```
//! gui.render(&mut factory, &mut stream);
//! ```

extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_text;
extern crate gfx_texture;
extern crate rustc_serialize;

pub mod element;
mod render;

use gfx::traits::*;
use std::cell::RefCell;
use std::rc::Rc;
use element::{TemplateElement, DomElement};

pub enum Event {
    MouseMoved([i32; 2]),
    MouseClick,
    Placeholder
}

/// Represents a Gui and provides tools to render it.
pub struct Gui<R: gfx::Resources, F: gfx::Factory<R>> {
    dom: DomElement,
    render_cache: Rc<RefCell<render::RenderCache<R, F>>>
}

impl<R: gfx::Resources, F: gfx::Factory<R> + Clone> Gui<R, F> {
    /// Initializes a new GUI with default values.
    pub fn new(factory: &mut F, template: TemplateElement) -> Gui<R, F> {
        Gui {
            dom: template.to_dom(),
            render_cache: Rc::new(RefCell::new(render::RenderCache::new(factory)))
        }
    }

    /// Raises an event in the GUI.
    pub fn raise_event<S: gfx::Stream<R>>(&mut self, stream: &S, event: Event) {
        // TODO: Calculate this using the latest frame's data, since that's what's visible at this point
        /*let (x, y) = stream.get_output().get_size();
        let area = render::RenderArea {
            position: [0, 0],
            size: [x as i32, y as i32]
        };

        self.root.raise_event(&event, &area);*/
    }

    /// Renders the Gui to the target stream.
    pub fn render<S: gfx::Stream<R>>(
        &mut self,
        stream: &mut S, factory: &mut F)
    {
        // Actually render the DOM
        render::render(stream, factory, self.render_cache.clone(), &self.dom);
    }
}
