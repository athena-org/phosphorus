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

extern crate phosphorus;
extern crate piston_window;

use piston_window::*;
use phosphorus::element::{TemplateElement};

fn main() {
    // Set up the Piston Window
    let (window, mut factory) = {
        let window: PistonWindow = WindowSettings::new("Hello Piston", [640, 480])
            .exit_on_esc(true)
            .into();

        let factory = window.factory.borrow_mut().clone();

        (window, factory)
    };

    // Set up our Phosphorus UI
    let mut gui = {
        let template = TemplateElement::new("layout")
            .with_attr("style_size", "[640, 480]")
            .with_attr("style_background", "[21, 23, 24]")
            .with_child(TemplateElement::new("text")
                .with_attr("value", "Hello world!")
                .with_attr("style_size", "[80, 20]")
            );

        phosphorus::Gui::new(&mut factory, template)
    };

    // Run the piston event loop
    for e in window {
        e.draw_3d(|stream| {
            // Render our actual GUI
            gui.render(stream, &mut factory);
        });
    }
}
