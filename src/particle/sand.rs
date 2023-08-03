use crate::{util::const_wrap, simulation::Simulation};
use super::{ElementMetadata, ElementTypeHint, ParticleSpawnFn, ParticleUpdateFn, ParticleDrawFn};

const SELF: &ElementMetadata = sand();

pub const fn sand() -> &'static ElementMetadata {
  &ElementMetadata {
    name: "Sand",
    type_hint: ElementTypeHint::Powder,
    color: 0xcfa668ff,
    density: 1000,
    heat_conductivity: 0.4,
    spawn: Some(const_wrap!(ParticleSpawnFn(|part| {
      let shading = fastrand::u8(..0x20);
      part.userdata = u32::from_be_bytes([shading, shading, shading, 0]);
    }))),
    update: Some(const_wrap!(ParticleUpdateFn(|sim, (x, y)| {
      let order = if fastrand::bool() { [0, 1, -1] } else { [0, -1, 1] };
      for ox in order {
        let current = (x, y);
        let desired = (x + ox, y + 1);
        if !Simulation::fits(desired) { continue }
        if sim.get(desired).element.meta().density < SELF.density {
          sim.swap(current, desired);
          break
        }
      }
    }))),
    draw: Some(ParticleDrawFn(|part, _time| {
      SELF.color - part.userdata
    })),
  }
}
