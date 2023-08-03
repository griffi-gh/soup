use crate::{simulation::Simulation, util::box_array};

pub struct SimulationRenderer {
  buffer: Box<[u32; Simulation::HEIGHT * Simulation::WIDTH]>,
}

impl SimulationRenderer {
  pub fn new() -> Self {
    Self {
      buffer: box_array![0; Simulation::HEIGHT * Simulation::WIDTH],
    }
  }
  pub fn buffer(&self) -> &[u8] {
    bytemuck::cast_slice(&self.buffer[..])
  }
  pub fn render(&mut self, sim: &Simulation) {
    for y in 0..Simulation::HEIGHT {
      for x in 0..Simulation::WIDTH {
        let particle = sim.get((x, y));
        let meta = particle.element.meta();
        let color = match meta.draw {
          Some(f) => f.0(particle, sim.frame()),
          None => meta.color,
        };
        self.buffer[y * Simulation::WIDTH + x] = color.swap_bytes();
      }
    }
  }
}
