use super::ParticleInfo;

pub const fn wall() -> &'static ParticleInfo {
  &ParticleInfo {
    name: "Wall",
    color: 0x666666ff,
    update: None,
  }
}
