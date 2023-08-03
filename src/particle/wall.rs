use super::{ElementMetadata, ElementTypeHint, ParticleDrawFn, heat_shading};

pub const fn wall() -> &'static ElementMetadata {
  &ElementMetadata {
    name: "Wall",
    type_hint: ElementTypeHint::Solid,
    color: 0x666666ff,
    density: u32::MAX,
    heat_conductivity: 0.85,
    spawn: None,
    update: None,
    draw: Some(ParticleDrawFn(heat_shading)),
  }
}
