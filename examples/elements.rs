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

use gfx::{Stream};
use phosphorus::element::{TemplateElement};

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
    let template = TemplateElement::new("layout")
        .with_attr("style_size", "[600, 500]")
        .with_attr("style_background", "[21, 23, 24]")
        // Start text
        .with_child(TemplateElement::new("text")
            .with_attr("value", "Hello world!")
            .with_attr("style_size", "[80, 20]")
        )
        // Horizontal colored blocks
        .with_child(TemplateElement::new("layout")
            .with_attr("style_size", "[200, 100]")
            .with_attr("style_layout_direction", "horizontal")
            .with_child(TemplateElement::new("color_block")
                .with_attr("style_background", "[200, 200, 0]")
                .with_child(TemplateElement::new("color_block")
                    .with_attr("style_size", "[40, 20]")
                    .with_attr("style_background", "[0, 200, 200]")
                )
                .with_child(TemplateElement::new("color_block")
                    .with_attr("style_size", "[70, 20]")
                    .with_attr("style_background", "[200, 0, 200]")
                )
            )
            .with_child(TemplateElement::new("color_block")
                .with_attr("style_background", "[0, 0, 200]")
                .with_child(TemplateElement::new("color_block")
                    .with_attr("style_size", "[70, 20]")
                    .with_attr("style_background", "[200, 0, 0]")
                )
                .with_child(TemplateElement::new("color_block")
                    .with_attr("style_size", "[40, 20]")
                    .with_attr("style_background", "[0, 200, 0]")
                )
            )
        )
        // End text
        .with_child(TemplateElement::new("text")
            .with_attr("value", "Hello from after the colored blocks!")
            .with_attr("style_size", "[80, 20]")
        );
    let mut gui = phosphorus::Gui::new(&mut factory, template);

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

        // Clear to remove any remaining data from the last frame
        stream.clear(gfx::ClearData {color: [1.0, 1.0, 1.0, 1.0], depth: 1.0, stencil: 0});

        // Render our actual GUI
        gui.render(&mut stream, &mut factory);

        // Show the rendered to buffer on the screen
        stream.present(&mut device);
    }
}
