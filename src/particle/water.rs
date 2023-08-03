use crate::{util::const_wrap, simulation::Simulation};
use super::{Element, ElementMetadata, ElementTypeHint, ParticleUpdateFn};

const SELF: &ElementMetadata = water();

pub const fn water() -> &'static ElementMetadata {
  &ElementMetadata {
    name: "Water",
    type_hint: ElementTypeHint::Liquid,
    color: 0x4e7dc3ff,
    density: 10,
    heat_conductivity: 0.75,
    spawn: None,
    update: Some(const_wrap!(ParticleUpdateFn(|sim, (x, y)| {
      if sim.get((x, y)).temperature > 1. {
        sim.get_mut((x, y)).element = Element::Steam;
        return
      }
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
        let current = (x, y);
        let desired = (x + offset.0, y + offset.1);
        if !Simulation::fits(desired) { continue }
        if sim.get(desired).element.meta().density < SELF.density {
          sim.swap(current, desired);
          break
        }
      }
    }))),
    draw: None,
  }
}
