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

use gfx;
use render;

mod image;
mod layout;
mod text;

pub use widgets::image::*;
pub use widgets::layout::*;
pub use widgets::text::*;

pub struct RenderArea {
    pub position: [u16;2],
    pub size: [u16;2]
}

pub struct RenderOffset {
    pub position: [u16;2]
}

pub trait Widget<R: gfx::Resources> {
    fn render(
        &self, data: &mut render::RenderData<R>,
        prev_area: &RenderArea, offset: &mut RenderOffset);
}
