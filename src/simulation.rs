use crate::{particle::Particle, util::box_array};

pub struct Simulation {
  state: Box<[[Particle; Self::WIDTH]; Self::HEIGHT]>,
  frame: u64,
}

impl Simulation {
  pub const WIDTH: usize = 800;
  pub const HEIGHT: usize = 600;

  pub fn new() -> Self {
    Self {
      state: box_array![[Particle::default(); Self::WIDTH]; Self::HEIGHT],
      frame: 0,
    }
  }

  pub fn frame(&self) -> u64 {
    self.frame
  }
  
  pub fn get(&self, pos: (usize, usize)) -> &Particle {
    &self.state[pos.1][pos.0]
  }

  pub fn get_mut(&mut self, pos: (usize, usize)) -> &mut Particle {
    &mut self.state[pos.1][pos.0]
  }

  pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
    if a == b { return }
    let x = self.state[a.1][a.0];
    self.state[a.1][a.0] = self.state[b.1][b.0];
    self.state[b.1][b.0] = x;
  }

  pub fn step(&mut self) {
    // Reset did_update flag
    for y in 0..Self::HEIGHT {
      for x in 0..Self::WIDTH {
        self.get_mut((x, y)).did_update = false;
      }
    }

    // Update everything
    let reverse_x = self.frame & 0b10 != 0;
    let reverse_y = self.frame & 0b01 != 0;
    for mut y in 0..Self::HEIGHT {
      if reverse_y { y = Self::HEIGHT - 1 - y }
      for mut x in 0..Self::WIDTH {
        if reverse_x { x = Self::WIDTH - 1 - x }
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
