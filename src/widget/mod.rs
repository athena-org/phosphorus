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

mod button;
mod image;
mod layout;
mod text;

pub use widget::button::*;
pub use widget::image::*;
pub use widget::layout::*;
pub use widget::text::*;

/// An interface for referencing to any kind of widget.
pub trait Widget<R: gfx::Resources> {
    /// Writes rendering data for this widget to `data`.
    fn render(
        &self, renderer: &mut render::Renderer<R>,
        prev_area: &render::RenderArea, offset: &mut render::RenderOffset);
}
