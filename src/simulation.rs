use rayon::prelude::*;
use crate::{particle::Particle, util::box_array};

pub type SimulationState = Box<[[Particle; Simulation::WIDTH]; Simulation::HEIGHT]>;

pub struct Simulation {
  state: SimulationState,
  frame: u64,
}

impl Simulation {
  pub const WIDTH: usize = 800;
  pub const HEIGHT: usize = 600;

  pub const IWIDTH: i32 = Self::WIDTH as _;
  pub const IHEIGHT: i32 = Self::HEIGHT as _;

  pub fn new() -> Self {
    Self {
      state: box_array![[Particle::default(); Self::WIDTH]; Self::HEIGHT],
      frame: 0,
    }
  }

  pub fn raw_state(&self) -> &SimulationState {
    &self.state
  }

  pub fn raw_state_mut(&mut self) -> &mut SimulationState {
    &mut self.state
  }

  pub fn frame(&self) -> u64 {
    self.frame
  }
  
  pub fn get(&self, pos: (i32, i32)) -> &Particle {
    &self.state[pos.1 as usize][pos.0 as usize]
  }

  pub fn get_mut(&mut self, pos: (i32, i32)) -> &mut Particle {
    &mut self.state[pos.1 as usize][pos.0 as usize]
  }

  pub fn fits(pos: (i32, i32)) -> bool {
    (0..Self::IWIDTH).contains(&pos.0) &&
    (0..Self::IHEIGHT).contains(&pos.1)
  }

  pub fn swap(&mut self, a: (i32, i32), b: (i32, i32)) {
    if a == b { return }
    let x = self.state[a.1 as usize][a.0 as usize];
    self.state[a.1 as usize][a.0 as usize] = self.state[b.1 as usize][b.0 as usize];
    self.state[b.1 as usize][b.0 as usize] = x;
  }

  pub fn step(&mut self) {
    // Reset did_update flag
    for y in 0..Self::IHEIGHT {
      for x in 0..Self::IWIDTH {
        self.get_mut((x, y)).did_update = false;
      }
    }

    // Update everything
    let reverse_x = self.frame & 0b10 != 0;
    let reverse_y = self.frame & 0b01 != 0;
    for mut y in 0..Self::IHEIGHT {
      if reverse_y { y = Self::IHEIGHT - 1 - y }
      for mut x in 0..Self::IWIDTH {
        if reverse_x { x = Self::IWIDTH - 1 - x }
        let particle = self.get_mut((x, y));
        let particle_info = particle.element.meta();
        if let Some(update_fn) = particle_info.update {
          if particle.did_update { continue }
          particle.did_update = true;
          update_fn.0(self, (x, y));
        }
      }
    }

    // Increment frame
    self.frame += 1;
  }
}
