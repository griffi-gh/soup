#[derive(Clone, Copy, Debug, Default)]
pub enum BrushShape {
  #[default]
  Rectangle,
  Ellipse,
}

#[derive(Clone, Copy, Debug)]
pub struct Brush {
  pub shape: BrushShape,
  pub position: (i32, i32),
  pub size: (u32, u32),
}

impl Default for Brush {
  fn default() -> Self {
    Self {
      shape: BrushShape::Ellipse,
      position: (0, 0),
      size: (15, 15),
    }
  }
}

impl Brush {
  pub fn iter(self) -> impl Iterator<Item = (i32, i32)> {
    let Brush { shape, position, size } = self;
    (position.0..(position.0 + size.0 as i32)).flat_map(move |x| {
      (position.1..(position.1 + size.1 as i32)).filter_map(move |y| {
        match shape {
          BrushShape::Rectangle => {
            Some((x, y))
          },
          BrushShape::Ellipse => {
            //convert stuff to float
            let fpixel: (f64, f64) = (x as f64, y as f64);
            let fpos: (f64, f64) = (position.0 as f64, position.1 as f64);
            let fsize: (f64, f64) = (size.0 as f64, size.1 as f64);

            //center of circle
            let fcenter: (f64, f64) = (fpos.0 + fsize.0 / 2., fpos.1 + fsize.1 / 2.);

            //Check if inside.... then return
            ((
              ((fpixel.0 - fcenter.0).powi(2) / (fsize.0 / 2.).powi(2)) +
              ((fpixel.1 - fcenter.1).powi(2) / (fsize.1 / 2.).powi(2))
            ) <= 1.).then_some((x, y))
          },
        }
      })
    })
  }

  //TODO line

  pub fn centered(self) -> Self {
    Brush {
      position: (
        (self.position.0 as f64 - self.size.0 as f64 / 2.).round() as i32,
        (self.position.1 as f64 - self.size.1 as f64 / 2.).round() as i32
      ),
      ..self
    }
  }
}
