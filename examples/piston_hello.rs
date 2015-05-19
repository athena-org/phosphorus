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

/*extern crate gfx;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate phosphorus;

use std::cell::RefCell;
use std::rc::Rc;
use piston::event::*;
use piston::window::{ AdvancedWindow, WindowSettings };
use piston_window::*;

fn main() {
    // Set up our Piston window
    let window = Rc::new(RefCell::new(glutin_window::GlutinWindow::new(
        opengl_graphics::OpenGL::_3_2,
        WindowSettings::new(
            "Phosphorus Piston Hello",
            [600, 500]
        )
        .exit_on_esc(true)
    )));
    let events = PistonWindow::new(window, empty_app());

    // Set up our Phosphorus UI
    let mut gui = {
        let mut canvas = events.canvas.borrow_mut();
        let root = phosphorus::widget::LayoutBuilder::new()
            .with_background_color([21, 23, 24])
            .with_widget(phosphorus::widget::TextBuilder::new()
                .with_text("Hello World!")
                .build_boxed())
            .with_widget(phosphorus::widget::TextBuilder::new()
                .with_text("Hello again, World!")
                .build_boxed())
            .with_widget(phosphorus::widget::ImageBuilder::new()
                .with_source("./examples/assets/test.png")
                .with_size([200, 200])
                .build_boxed(&mut canvas.factory))
            .with_widget(phosphorus::widget::TextBuilder::new()
                .with_text("Hello from after the image!")
                .build_boxed())
            .build();
        phosphorus::Gui::new(&mut canvas.factory, root)
    };

    for e in events {
        if let Some(_) = e.render_args() {
            e.draw_3d(|canvas| {
                // Render our actual GUI
                let mut stream = (&mut canvas.renderer, &canvas.output);
                gui.render(&mut canvas.factory, &mut stream);
            });
        }

        if let Some(_) = e.update_args() {
            // Update Here
        }
    }
}*/
