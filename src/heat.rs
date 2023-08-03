use rayon::prelude::*;
use std::f32::consts::FRAC_1_SQRT_2;
use crate::{simulation::Simulation, util::box_array};

pub struct HeatSimulation {
  buffer: Box<[[f32; Simulation::WIDTH]; Simulation::HEIGHT]>
}

impl HeatSimulation {
  pub const ITERATIONS: usize = 4;

  pub fn new() -> Self {
    Self {
      buffer: box_array![[0; Simulation::WIDTH]; Simulation::HEIGHT],
    }
  }

  fn update_buffer(&mut self, sim: &Simulation) {
    for y in 0..Simulation::HEIGHT {
      for x in 0..Simulation::WIDTH {
        let particle = sim.get((x as i32, y as i32));
        self.buffer[y][x] = particle.temperature;
      }
    }
  }

  pub fn step(&mut self, sim: &mut Simulation) {
    //Propagate heat
    //TODO: early exit
    for _  in 0..Self::ITERATIONS {
      self.update_buffer(sim);
      sim.raw_state_mut()
        .par_iter_mut()
        .enumerate()
        .for_each(|(y, sim_row)| {
          for (x, particle) in sim_row.iter_mut().enumerate() {
            let neighbour = |ox: i32, oy: i32| {
              let x = x as i32 + ox;
              let y = y as i32 + oy;
              if !Simulation::fits((x, y)) { return 0. }
              self.buffer[y as usize][x as usize]
            };
            let neighbours_temp = 0.125 * (
              neighbour( 1, -1) * FRAC_1_SQRT_2 +
              neighbour( 1,  0) +
              neighbour( 1,  1) * FRAC_1_SQRT_2 +
              neighbour( 0, -1) +
              neighbour( 0,  1) +
              neighbour(-1, -1) * FRAC_1_SQRT_2 +
              neighbour(-1,  0) +
              neighbour(-1,  1) * FRAC_1_SQRT_2
            );
            let error = neighbours_temp - particle.temperature;
            let heat_conductivity = particle.element.meta().heat_conductivity;
            particle.temperature += heat_conductivity * error;
          }
        });
    }
  }
}
