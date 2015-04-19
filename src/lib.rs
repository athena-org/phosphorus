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

#![feature(plugin, custom_attribute)]
#![plugin(gfx_macros)]

extern crate gfx;

use gfx::traits::*;

pub fn say_hello<
    R: gfx::Resources,
    O: gfx::Output<R>,
    C: gfx::CommandBuffer<R>
>(
    output: &O,
    renderer: &mut gfx::Renderer<R, C>)
{
    let mask = output.get_mask();
    renderer.clear(gfx::ClearData {
        color: [0.3, 0.3, 1.0, 1.0],
        depth: 1.0,
        stencil: 0,
    }, mask, output);
}
