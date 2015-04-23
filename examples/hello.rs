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

static HELLO_MARKUP: &'static str = include_str!("hello-markup.jade");

fn main() {
    // Set up our window
    let window = glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(1280, 720)
        .with_title(String::from("Hello"))
        .build_strict().unwrap();
    let mut canvas = gfx_window_glutin::init(window).into_canvas();

    // Set up our phosphorus gui
    let root = phosphorus::widgets::LayoutWidgetBuilder::new()
        .with_background_color([0.082, 0.090, 0.094])
        .with_widget(phosphorus::widgets::TextWidgetBuilder::new()
            .with_text("Hello World!")
            .build_boxed())
        .with_widget(phosphorus::widgets::TextWidgetBuilder::new()
            .with_text("Hello again, World!")
            .build_boxed())
        .build();
    let mut gui = phosphorus::Gui::new(&mut canvas.factory, root);

    'main: loop {
        // Quit when the window is closed
        for event in canvas.output.window.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => (),
            }
        }
        
        // Render our actual GUI
        gui.render(&mut canvas.output, &mut canvas.renderer, &mut canvas.factory);

        // Show the rendered to buffer on the screen
        canvas.present();
    }
}
