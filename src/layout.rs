use gfx;
use gfx::traits::*;

pub struct GuiLayout {
    background: BackgroundType
}

pub struct LayoutOffset {
    pub position: [u16;2],
    pub size: [u16;2]
}

pub enum BackgroundType {
    None,
    Color([f32;3])
}

impl GuiLayout {
    pub fn set_background(&mut self, background: BackgroundType) {
        self.background = background;
    }
}

impl GuiLayout {
    pub fn new() -> GuiLayout {
        GuiLayout {
            background: BackgroundType::None
        }
    }

    pub fn render<
        R: gfx::Resources,
        O: gfx::Output<R>,
        C: gfx::CommandBuffer<R>
    >(
        &self,
        output: &O,
        renderer: &mut gfx::Renderer<R, C>,
        prev_offset: &LayoutOffset)
    {
    }
}
