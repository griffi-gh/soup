use crate::{
  particle::{ParticleInfo, Element, ParticleUpdateFn},
  util::const_wrap,
};

pub const fn sand() -> &'static ParticleInfo {
  &ParticleInfo {
    name: "Sand",
    color: 0xcfa668ff,
    update: Some(const_wrap!(ParticleUpdateFn(|sim, (x, y)| {
      //Fall down
      let order = if fastrand::bool() { [0, 1, -1] } else { [0, -1, 1] };
      for ox in order {
        let desired = (x.wrapping_add_signed(ox), y + 1);
        let current = (x, y);
        if sim.get(desired).element == Element::Air {
          sim.swap(current, desired);
          break
        }
      }
    }))),
  }
}
