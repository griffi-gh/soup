use super::{ElementMetadata, ElementTypeHint};

pub const fn air() -> &'static ElementMetadata {
  &ElementMetadata {
    name: "Air",
    type_hint: ElementTypeHint::Air,
    color: 0x00000000,
    density: 0,
    heat_conductivity: 0.01,
    spawn: None,
    update: None,
    draw: None,
  }
}
