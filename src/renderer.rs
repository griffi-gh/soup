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
        self.buffer[y * Simulation::WIDTH + x] = sim.get((x, y)).kind.info().color.swap_bytes();
      }
    }
  }
}
