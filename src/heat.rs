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
        let particle = sim.get((x, y));
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
            let try_get = |y: usize, x: usize| {
              self.buffer.get(y).and_then(|r| r.get(x)).copied()
            };
            let try_get_or_zero = |y: usize, x: usize| {
              try_get(y, x).unwrap_or(0.)
            };
            let y_up = y.wrapping_sub(1);
            let y_down = y.wrapping_add(1);
            let x_left = x.wrapping_sub(1);
            let x_right = x.wrapping_add(1);
            let neighbours_temp = 0.125 * (
              try_get_or_zero(y_up, x_left) * FRAC_1_SQRT_2 +
              try_get_or_zero(y_up, x) +
              try_get_or_zero(y_up, x_right) * FRAC_1_SQRT_2 +
              try_get_or_zero(y, x_left) +
              try_get_or_zero(y, x_right) +
              try_get_or_zero(y_down, x_left) * FRAC_1_SQRT_2 +
              try_get_or_zero(y_down, x) +
              try_get_or_zero(y_down, x_right) * FRAC_1_SQRT_2
            );
            let error = neighbours_temp - particle.temperature;
            let heat_conductivity = particle.element.meta().heat_conductivity;
            particle.temperature += heat_conductivity * error;
          }
        });
    }
  }
}
