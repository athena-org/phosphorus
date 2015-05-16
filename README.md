<!--
    Copyright 2015 The Athena Developers.

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
-->

# Phosphorus [![Build Status](https://travis-ci.org/athena-org/phosphorus.png?branch=develop)](https://travis-ci.org/gfx-rs/gfx-rs)

Phosphorus is a user interface library written in rust. It is written for use in Athena, but can also be used in [Piston](https://github.com/PistonDevelopers/piston) or standalone. It can be integrated into any application that uses [gfx-rs](https://github.com/gfx-rs/gfx-rs).

To use Phosphorus you need to create a `Gui` object, filling it with a layout.

```Rust
let root = phosphorus::widget::LayoutBuilder::new()
    .with_background_color([21, 23, 24])
    .build();
let mut gui = phosphorus::Gui::new(&mut canvas.factory, root);
```

Then you can render it using a gfx `Factory` and `Stream` combination.

```Rust
gui.render(&mut factory, &mut stream);
```

## Goals
Phosphorus aims to:
- Make it easy to create complex layout-based UIs
- Provide a non-immediate-mode alternative to [Conrod](https://github.com/PistonDevelopers/conrod)
- Provide a UI library for both desktop applications and games
- Provide markup based UI (Not Done)
- Support runtime updating of the UI through data binding (Not Done)
