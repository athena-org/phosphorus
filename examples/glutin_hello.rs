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
extern crate gfx_device_gl;
extern crate gfx_window_glutin;

use gfx::traits::*;
use phosphorus::widget::*;

//static HELLO_MARKUP: &'static str = include_str!("assets/hello-markup.jade");

fn main() {
    // Set up our window
    let (mut stream, mut device, mut factory) = {
        let window = glutin::WindowBuilder::new()
            .with_vsync()
            .with_dimensions(600, 500)
            .with_title(String::from("Phosphorus Widgets"))
            .build_strict().unwrap();
        gfx_window_glutin::init(window)
    };

    // Set up our Phosphorus UI
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
            .build_boxed(&mut factory))
        .with_widget(phosphorus::widget::TextBuilder::new()
            .with_text("Hello from after the image!")
            .build_boxed())
        .with_widget(phosphorus::widget::ButtonBuilder::new()
            .with_text("Click me?")
            .with_callback(Box::new(|| println!("Hello")))
            .build_boxed())
        .build();
    let mut gui = phosphorus::Gui::new(&mut device, &mut factory, root);

    // Run our actual UI loop
    'main: loop {
        // Quit when the window is closed
        for event in stream.out.window.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                glutin::Event::MouseMoved((x, y)) =>
                    gui.raise_event(&stream, phosphorus::Event::MouseMoved([x, y])),
                glutin::Event::MouseInput(glutin::ElementState::Released, _) =>
                    gui.raise_event(&stream, phosphorus::Event::MouseClick),
                _ => (),
            }
        }

        stream.clear(gfx::ClearData {color: [1.0, 1.0, 1.0, 1.0], depth: 1.0, stencil: 0});

        {
            // Render our actual GUI
            gui.render(&mut factory, &mut stream);
        }

        // Show the rendered to buffer on the screen
        //stream.present(&mut device); ICE!
        stream.flush(&mut device);
        stream.out.window.swap_buffers();
        device.cleanup();
    }
}
