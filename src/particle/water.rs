use crate::{
  particle::{ParticleInfo, Element, ParticleUpdateFn},
  util::const_wrap,
};

pub const fn water() -> &'static ParticleInfo {
  &ParticleInfo {
    name: "Water",
    color: 0x4e7dc3ff,
    update: Some(const_wrap!(ParticleUpdateFn(|sim, (x, y)| {
      //Fall down
      let order = if fastrand::bool() { [
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 0),
        (-1, 1),
      ] } else { [
        (0, 1),
        (-1, 0),
        (-1, 1),
        (1, 0),
        (1, 1),
      ] };
      for offset in order {
        let desired = (x.wrapping_add_signed(offset.0), y.wrapping_add_signed(offset.1));
        let current = (x, y);
        if sim.get(desired).element == Element::Air {
          sim.swap(current, desired);
          break
        }
      }
    }))),
  }
}
