use crate::Simulation;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ParticleUpdateFn(pub fn(&mut Simulation, (usize, usize)));

#[derive(Clone, Copy, Debug)]
pub struct ParticleInfo {
  pub name: &'static str,
  pub color: u32,
  pub update: Option<ParticleUpdateFn>,
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
      const ALL: &'static [Self] = &[
        $(Self::$enum_field),*
      ];

      pub const fn info(self) -> &'static ParticleInfo {
        match self {
          $($enum::$enum_field => $mod_name::$mod_name()),*
        }
      }
    }
  };
}

particles! {
  ParticleKind {
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
  pub kind: ParticleKind,
  pub did_update: bool,
  pub userdata: u32,
}

impl Particle {
  pub fn new(kind: ParticleKind) -> Self {
    Self {
      kind,
      ..Default::default()
    }
  }
}
