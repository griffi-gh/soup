use crate::{util::const_wrap, simulation::Simulation};
use super::{ElementMetadata, ElementTypeHint, ParticleSpawnFn, ParticleUpdateFn, Element};

const SELF: &ElementMetadata = fire();

pub const fn fire() -> &'static ElementMetadata {
  &ElementMetadata {
    name: "Fire",
    type_hint: ElementTypeHint::Gas,
    color: 0xff0000ff,
    density: 1,
    heat_conductivity: 1.,
    spawn: Some(const_wrap!(ParticleSpawnFn(|part| {
      part.temperature = 400.;
    }))),
    update: Some(const_wrap!(ParticleUpdateFn(|sim, (x, y)| {
      if sim.get((x, y)).temperature < 300. {
        sim.get_mut((x, y)).element = Element::Air;
        return
      }
      let order = if fastrand::bool() { [0, 1, -1] } else { [0, -1, 1] };
      for ox in order {
        let current = (x, y);
        let desired = (x + ox, y - 1);
        if !Simulation::fits(desired) { continue }
        if sim.get(desired).element.meta().density < SELF.density {
          sim.swap(current, desired);
        }
      }
    }))),
    draw: None,
  }
}
