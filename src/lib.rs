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
//! let mut gui = phosphorus::Gui::new(&mut canvas.factory, root, |d: &mut gfx_device_gl::Device| d.spawn_factory());
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

use gfx::traits::*;
use std::cell::RefCell;
use std::rc::Rc;

mod render;
pub mod widget;

pub enum Event {
    MouseMoved([i32; 2]),
    MouseClick,
    Placeholder
}

/// Represents a Gui and provides tools to render it.
pub struct Gui<R: gfx::Resources, F: gfx::Factory<R>> {
    root: widget::Layout<R>,
    render_data: Rc<RefCell<render::RenderData<R, F>>>
}

impl<R: gfx::Resources, F: gfx::Factory<R>> Gui<R, F> {
    /// Initializes a new Gui with default values.
    pub fn new<D: gfx::Device, FactorySpawner>(device: &mut D, root: widget::Layout<R>, spawner: FactorySpawner) -> Gui<R, F>
        where FactorySpawner: Fn(&mut D) -> F
    {
        Gui {
            root: root,
            render_data: Rc::new(RefCell::new(render::RenderData::new(device, spawner)))
        }
    }

    /// Gets the root layout as immutable.
    pub fn root(&self) -> &widget::Layout<R> { &self.root }
    /// Gets the root layout as mutable.
    pub fn root_mut(&mut self) -> &mut widget::Layout<R> { &mut self.root }
    /// Sets the root layout.
    pub fn set_root(&mut self, root: widget::Layout<R>) { self.root = root; }

    pub fn raise_event<S: gfx::Stream<R>>(&mut self, stream: &S, event: Event) {
        let (x, y) = stream.get_output().get_size();
        let area = render::RenderArea {
            position: [0, 0],
            size: [x as i32, y as i32]
        };

        self.root.raise_event(&event, &area);
    }

    pub fn render<S: gfx::Stream<R>>(
        &mut self,
        factory: &mut F, stream: &mut S)
    {
        // Set up a layout area to the whole screen
        let (x, y) = stream.get_output().get_size();
        let area = render::RenderArea {
            position: [0, 0],
            size: [x as i32, y as i32]
        };

        // Actually tell the root layout to render to the data
        let mut renderer = render::ConcreteRenderer::new(factory, stream, self.render_data.clone(), &area);
        self.root.render(&mut renderer, &area);
    }
}
