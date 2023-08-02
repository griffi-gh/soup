use super::ParticleInfo;

pub const fn air() -> &'static ParticleInfo {
  &ParticleInfo {
    name: "Air",
    color: 0x00000000,
    update: None,
  }
}
