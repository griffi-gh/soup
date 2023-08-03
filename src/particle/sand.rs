use crate::{
  particle::{ElementMetadata, Element, ParticleSpawnFn, ParticleUpdateFn, ParticleDrawFn},
  util::const_wrap,
};

const SELF: &ElementMetadata = sand();

pub const fn sand() -> &'static ElementMetadata {
  &ElementMetadata {
    name: "Sand",
    color: 0xcfa668ff,
    density: 1000,
    spawn: Some(const_wrap!(ParticleSpawnFn(|part| {
      let shading = fastrand::u8(..0x20);
      part.userdata = u32::from_be_bytes([shading, shading, shading, 0]);
    }))),
    update: Some(const_wrap!(ParticleUpdateFn(|sim, (x, y)| {
      let order = if fastrand::bool() { [0, 1, -1] } else { [0, -1, 1] };
      for ox in order {
        let desired = (x.wrapping_add_signed(ox), y + 1);
        let current = (x, y);
        if sim.get(desired).element.meta().density < SELF.density {
          sim.swap(current, desired);
          break
        }
      }
    }))),
    draw: Some(const_wrap!(ParticleDrawFn(|part, _time| {
      SELF.color - part.userdata
    }))),
  }
}
