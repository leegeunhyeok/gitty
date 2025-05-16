const DISPLAY_WIDTH: u32 = 320;
const DISPLAY_HEIGHT: u32 = 170;

#[cfg(feature = "simulator")]
pub mod implements {
    use super::*;
    use mousefood::{
        embedded_graphics::geometry, prelude::Bgr565, simulator::SimulatorDisplay, EmbeddedBackend,
        EmbeddedBackendConfig,
    };

    pub type Display = SimulatorDisplay<Bgr565>;

    pub fn get_display() -> Display {
        SimulatorDisplay::<Bgr565>::new(geometry::Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT))
    }

    pub fn to_backend(display: &mut Display) -> EmbeddedBackend<'_, Display, Bgr565> {
        EmbeddedBackend::new(display, EmbeddedBackendConfig::default())
    }
}

#[cfg(not(feature = "simulator"))]
pub mod implements {}
