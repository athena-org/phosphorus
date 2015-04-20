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
    let mut gui = phosphorus::Gui::new(&mut canvas.factory);
    {
        let mut root = gui.root_mut();
        root.set_background(phosphorus::layout::BackgroundType::Color([0.9, 0.3, 0.3]));
    }

    'main: loop {
        // quit when Esc is pressed.
        for event in canvas.output.window.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => {},
            }
        }

        // Clear in so we don't have lingering stuff from the last frame
        canvas.clear(gfx::ClearData {
            color: [0.3, 0.3, 0.9, 1.0],
            depth: 1.0,
            stencil: 0,
        });

        gui.render(&canvas.output, &mut canvas.renderer);

        canvas.present();
    }
}
