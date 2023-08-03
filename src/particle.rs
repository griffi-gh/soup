use crate::Simulation;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ParticleSpawnFn(pub fn(&mut Particle));

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ParticleUpdateFn(pub fn(&mut Simulation, (i32, i32)));

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ParticleDrawFn(pub fn(&Particle, u64) -> u32);

pub fn heat_shading(part: &Particle, _frame: u64) -> u32 {
  let heat_ratio = (part.temperature / 1000.).clamp(0., 1.);

  //XXX: is there a better way to write this?
  let [r, g, b, a] = u32::to_be_bytes(part.element.meta().color);
  u32::from_be_bytes([
    (r as f32 * (1. - heat_ratio) + (255. * heat_ratio)) as u8,
    (g as f32 * (1. - heat_ratio) + 0.) as u8,
    (b as f32 * (1. - heat_ratio) + 0.) as u8,
    (a as f32) as u8
  ])
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ElementTypeHint {
  Air,
  Gas,
  Liquid,
  Powder,
  Solid,
}

#[derive(Clone, Copy, Debug)]
pub struct ElementMetadata {
  pub name: &'static str,
  pub type_hint: ElementTypeHint,
  pub color: u32,
  pub density: u32,
  pub heat_conductivity: f32,
  pub spawn: Option<ParticleSpawnFn>,
  pub update: Option<ParticleUpdateFn>,
  pub draw: Option<ParticleDrawFn>
}

impl ElementMetadata {
  pub const fn default() -> Self {
    Self {
      name: "<default>",
      type_hint: ElementTypeHint::Solid,
      color: 0xffffffff,
      density: 0,
      heat_conductivity: 0.,
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
    Fire: mod fire,
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
  pub temperature: f32,
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
