#[derive(Clone, Copy, Debug, Default)]
pub enum BrushShape {
  #[default]
  Square,
  Circle,
}

#[derive(Clone, Copy, Debug)]
pub struct Brush {
  pub shape: BrushShape,
  pub position: (usize, usize),
  pub size: (usize, usize),
}

impl Default for Brush {
  fn default() -> Self {
    Self {
      shape: BrushShape::Circle,
      position: (0, 0),
      size: (15, 15),
    }
  }
}

impl Brush {
  pub fn iter(self) -> impl Iterator<Item = (usize, usize)> {
    let Brush { shape: kind, position, size } = self;
    (position.0..(position.0 + size.0)).flat_map(move |x| {
      (position.1..(position.1 + size.1)).filter_map(move |y| {
        match kind {
          BrushShape::Square => Some((x, y)),
          BrushShape::Circle => {
            assert_eq!(size.0, size.1, "Ellipse not supported yet");
            let fpixel: (f64, f64) = (x as f64, y as f64);
            let fpos: (f64, f64) = (position.0 as f64, position.1 as f64);
            let fsize: (f64, f64) = (size.0 as f64, size.1 as f64);
            let fcenter: (f64, f64) = (fpos.0 + fsize.0 / 2., fpos.1 + fsize.1 / 2.);
            let vec_len = ((fpixel.0 - fcenter.0).powi(2) + (fpixel.1 - fcenter.1).powi(2)).sqrt();
            (vec_len <= fsize.0 / 2.).then_some((x, y))
          },
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
