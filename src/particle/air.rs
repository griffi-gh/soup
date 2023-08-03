use super::ElementMetadata;

pub const fn air() -> &'static ElementMetadata {
  &ElementMetadata {
    name: "Air",
    color: 0x00000000,
    density: 0,
    spawn: None,
    update: None,
    draw: None,
  }
}
