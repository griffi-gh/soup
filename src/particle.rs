use crate::Simulation;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ParticleSpawnFn(pub fn(&mut Particle));

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ParticleUpdateFn(pub fn(&mut Simulation, (usize, usize)));

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ParticleDrawFn(pub fn(&Particle, u64) -> u32);

#[derive(Clone, Copy, Debug)]
pub struct ElementMetadata {
  pub name: &'static str,
  pub color: u32,
  pub density: u32,
  pub spawn: Option<ParticleSpawnFn>,
  pub update: Option<ParticleUpdateFn>,
  pub draw: Option<ParticleDrawFn>
}

impl ElementMetadata {
  pub const fn default() -> Self {
    Self {
      name: "<default>",
      color: 0xffffffff,
      density: 0,
      spawn: None,
      update: None,
      draw: None,
    }
  }
}

impl Default for ElementMetadata {
  fn default() -> Self {
    Self::default()
  }
}

macro_rules! particles {
  {
    $enum: ident {
      $($enum_field:ident : mod $mod_name: tt $(,)* $(: id $id: literal)*),*
    }
  } => {
    $(
      mod $mod_name;
    )*

    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum $enum {
      #[default]
      $($enum_field $(= $id)*),*
    }

    impl $enum {
      pub const ALL: &'static [Self] = &[
        $(Self::$enum_field),*
      ];

      pub const fn meta(self) -> &'static ElementMetadata {
        match self {
          $($enum::$enum_field => $mod_name::$mod_name()),*
        }
      }
    }
  };
}

particles! {
  Element {
    Air: mod air,
    Sand: mod sand,
    Wall: mod wall,
    Water: mod water,
  }
}

// #[bitfield(filled = false)]
// #[derive(Clone, Copy, Debug, Default)]
// pub struct ParticleFlags {
//   pub did_update: bool,
// }

#[derive(Clone, Copy, Debug, Default)]
pub struct Particle {
  pub element: Element,
  pub did_update: bool,
  pub temperature: i16,
  pub userdata: u32,
}

impl Particle {
  pub fn spawn(element: Element) -> Self {
    let mut particle = Self {
      element,
      ..Default::default()
    };
    if let Some(ParticleSpawnFn(spawn_fn)) = element.meta().spawn {
      spawn_fn(&mut particle);
    }
    particle
  }
}
