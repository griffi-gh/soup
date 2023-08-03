use crate::{util::const_wrap, simulation::Simulation};
use super::{ElementMetadata, ElementTypeHint, ParticleSpawnFn, ParticleUpdateFn, Element};

const SELF: &ElementMetadata = steam();

//TODO fix steam

pub const fn steam() -> &'static ElementMetadata {
  &ElementMetadata {
    name: "Steam",
    type_hint: ElementTypeHint::Gas,
    color: 0xddddddff,
    density: 5,
    heat_conductivity: 0.5,
    spawn: Some(const_wrap!(ParticleSpawnFn(|part| {
      part.temperature = 150.;
    }))),
    update: Some(const_wrap!(ParticleUpdateFn(|sim, (x, y)| {
      if sim.get((x, y)).temperature < 60. {
        sim.get_mut((x, y)).element = Element::Water;
        return
      }
      let current = (x, y);
      let desired = (x, y - 1);
      if Simulation::fits(desired) {
        //XXX: this *is* incorrect
        let desired_meta = sim.get(desired).element.meta();
        if desired_meta.type_hint < ElementTypeHint::Solid {
          sim.swap(current, desired);
        }
      }
    }))),
    draw: None,
  }
}
