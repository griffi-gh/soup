#[derive(Clone, Copy, Debug, Default)]
pub enum BrushKind {
  #[default]
  Square,
  Circle,
}

#[derive(Clone, Copy, Debug)]
pub struct Brush {
  pub kind: BrushKind,
  pub position: (usize, usize),
  pub size: (usize, usize),
}

impl Default for Brush {
  fn default() -> Self {
    Self {
      kind: BrushKind::Square,
      position: (0, 0),
      size: (15, 15),
    }
  }
}

impl Brush {
  pub fn iter(self) -> impl Iterator<Item = (usize, usize)> {
    let Brush { kind, position, size } = self;
    (position.0..(position.0 + size.0)).flat_map(move |x| {
      (position.1..(position.1 + size.1)).filter_map(move |y| {
        match kind {
          BrushKind::Square => Some((x, y)),
          BrushKind::Circle => unimplemented!(),
        }
      })
    })
  }
  pub fn centered(self) -> Self {
    Brush {
      position: (
        (self.position.0 as f64 - self.size.0 as f64 / 2.).round() as usize,
        (self.position.1 as f64 - self.size.1 as f64 / 2.).round() as usize
      ),
      ..self
    }
  }
}
