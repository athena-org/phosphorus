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
extern crate glutin;
extern crate gfx;
extern crate gfx_window_glutin;

use gfx::traits::*;

static HELLO_MARKUP: &'static str = include_str!("assets/hello-markup.jade");

fn main() {
    // Set up our window
    let window = glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(600, 500)
        .with_title(String::from("Phosphorus Widgets"))
        .build_strict().unwrap();
    let mut canvas = gfx_window_glutin::init(window).into_canvas();
    canvas.output.set_gamma(gfx::Gamma::Original).unwrap();

    // Set up our Phosphorus UI
    let root = phosphorus::widget::LayoutWidgetBuilder::new()
        .with_background_color([21, 23, 24])
        .with_widget(phosphorus::widget::TextWidgetBuilder::new()
            .with_text("Hello World!")
            .build_boxed())
        .with_widget(phosphorus::widget::TextWidgetBuilder::new()
            .with_text("Hello again, World!")
            .build_boxed())
        .with_widget(phosphorus::widget::ImageWidgetBuilder::new()
            .with_source("./examples/assets/test.png")
            .with_size([200, 200])
            .build_boxed(&mut canvas.factory))
        .with_widget(phosphorus::widget::TextWidgetBuilder::new()
            .with_text("Hello from after the image!")
            .build_boxed())
        .build();
    let mut gui = phosphorus::Gui::new(&mut canvas.factory, root);

    // Run our actual UI loop
    'main: loop {
        // Quit when the window is closed
        for event in canvas.output.window.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => (),
            }
        }

        canvas.clear(gfx::ClearData {color: [1.0, 1.0, 1.0, 1.0], depth: 1.0, stencil: 0});

        {
            // Render our actual GUI
            let mut stream = (&mut canvas.renderer, &canvas.output);
            gui.render(&mut canvas.factory, &mut stream);
        }

        // Show the rendered to buffer on the screen
        canvas.present();
    }
}
