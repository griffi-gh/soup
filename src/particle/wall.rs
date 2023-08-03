use super::ElementMetadata;

pub const fn wall() -> &'static ElementMetadata {
  &ElementMetadata {
    name: "Wall",
    color: 0x666666ff,
    density: 0,
    spawn: None,
    update: None,
    draw: None,
  }
}
