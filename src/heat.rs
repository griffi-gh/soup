use itertools::Itertools;
use rayon::prelude::*;
use std::f32::consts::FRAC_1_SQRT_2;
use crate::{simulation::Simulation, util::box_array, particle::Particle};

#[derive(Clone, Copy, Debug, Default)]
struct HeatBufferEntry {
  pub temperature: f32,
  pub conductivity: f32,
}

pub struct HeatSimulation {
  buffer: Box<[[HeatBufferEntry; Simulation::WIDTH]; Simulation::HEIGHT]>
}

impl HeatSimulation {
  pub const ITERATIONS: usize = 4;

  pub fn new() -> Self {
    Self {
      buffer: box_array![[HeatBufferEntry::default(); Simulation::WIDTH]; Simulation::HEIGHT],
    }
  }

  fn update_buffer(&mut self, sim: &Simulation) {
    for y in 0..Simulation::HEIGHT {
      for x in 0..Simulation::WIDTH {
        let particle = *sim.get((x as i32, y as i32));
        self.buffer[y][x] = HeatBufferEntry {
          temperature: particle.temperature,
          conductivity: particle.element.meta().heat_conductivity,
        };
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
            let neighbour = |ox, oy| {
              let x = x as i32 + ox;
              let y = y as i32 + oy;
              Simulation::fits((x, y)).then(|| self.buffer[y as usize][x as usize])
            };

            let current = HeatBufferEntry {
              temperature: particle.temperature,
              conductivity: particle.element.meta().heat_conductivity,
            };
            
            for ox in -1..=1 {
              for oy in -1..=1 {
                if ox == 0 && oy == 0 { continue }
                let neighbour = neighbour(ox, oy).unwrap_or_default();
                let inv_distance = if ox.abs() == oy.abs() { FRAC_1_SQRT_2 } else { 1. };
                let temperature_difference = neighbour.temperature - current.temperature;
                //XXX: multiply or average?
                //let common_conductivity = (neighbour.conductivity + current.conductivity) / 2.;
                let common_conductivity = neighbour.conductivity * current.conductivity;
                let solution = temperature_difference * common_conductivity * inv_distance;
                particle.temperature += solution / 8.;
              }
            }
          }
        });
    }
  }
}
