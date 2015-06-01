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

use std::rc::{Rc};
use render;
use runtime::{RuntimeNode};
use template;
use template::{LayoutBackground};
use gfx;

pub struct LayoutArea {
    pub position: [i32; 2],
    pub size: [i32; 2]
}

pub struct Layout {
    children: Vec<Box<RuntimeNode>>,
    background: LayoutBackground
}

impl Layout {
    pub fn new(template: Rc<template::Layout>) -> Layout {
        let children: Vec<_> = template.children().into_iter()
            .map(|c| c.create_runtime())
            .collect();

        Layout {
            children: children,
            background: template.background()
        }
    }

    pub fn render<R: gfx::Resources>(
        &self,
        renderer: &mut render::Renderer<R>, parent_area: &LayoutArea)
    {
        self.render_background(renderer, parent_area);
    }

    fn render_background<R: gfx::Resources>(
        &self,
        renderer: &mut render::Renderer<R>,
        area: &LayoutArea)
    {
        match self.background {
            // Different background types render differently
            LayoutBackground::None => {},
            LayoutBackground::Color(c) => {
                renderer.render_rect_flat(area.position, area.size, c);
            }
        }
    }
}

impl RuntimeNode for Layout {
}
