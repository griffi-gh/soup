use winit::{
  window::WindowBuilder,
  dpi::LogicalSize,
  event_loop::EventLoop,
  event::{Event, VirtualKeyCode},
};
use winit_input_helper::WinitInputHelper;
use pixels::{Pixels, SurfaceTexture};

pub(crate) mod util;
pub(crate) mod particle;
pub(crate) mod simulation;
pub(crate) mod renderer;
pub(crate) mod brush;

use particle::{Particle, Element};
use simulation::Simulation;
use renderer::SimulationRenderer;
use brush::Brush;

fn main() {
  let size = LogicalSize::new(Simulation::WIDTH as u32, Simulation::HEIGHT as u32);
  
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("soup")
    .with_inner_size(size)
    .with_min_inner_size(size)
    .build(&event_loop)
    .unwrap();
  let mut input = WinitInputHelper::new();

  let mut pixels = {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    Pixels::new(Simulation::WIDTH as u32, Simulation::HEIGHT as u32, surface_texture).unwrap()
  };
  
  let mut sim = Simulation::new();
  let mut ren = SimulationRenderer::new();
  
  for x in 100..700 {
    for y in 550..560  {
      *sim.get_mut((x, y)) = Particle::new(Element::Wall);
    }
  }

  let mut brush = Brush::default();
  let mut selection = Element::Sand;

  event_loop.run(move |event, _, control_flow| {
    control_flow.set_poll();

    if let Event::RedrawRequested(_) = event {
      ren.render(&sim);
      pixels.frame_mut().copy_from_slice(ren.buffer());
      pixels.render().unwrap();
    }

    if input.update(&event) {
      // Exit if requested
      if input.close_requested() {
        control_flow.set_exit();
        return
      }

      // Resize
      if let Some(size) = input.window_resized() {
        pixels.resize_surface(size.width, size.height).unwrap();
      }

      // Hadle element selection
      const KEY_ELEMENT_MAP: &[(VirtualKeyCode, Element)] = &[
        (VirtualKeyCode::Key1, Element::Sand),
        (VirtualKeyCode::Key2, Element::Water),
        (VirtualKeyCode::Key3, Element::Wall),
      ];
      for (key, elem) in KEY_ELEMENT_MAP {
        if input.key_pressed(*key) {
          selection = *elem;
        }
      }

      // Handle brush/mouse
      if let Some(mouse) = input.mouse() {
        brush.position = (mouse.0 as usize, mouse.1 as usize);
      }
      if input.mouse_held(0) || input.mouse_held(1) {
        let elem = if input.mouse_held(0) { selection } else { Element::Air };
        for pos in brush.centered().iter() {
          let particle = sim.get_mut(pos);
          if particle.element == Element::Air || elem == Element::Air {
            *particle = Particle::new(elem);
          }
        }
      }
      if !input.held_control() {
        brush.size.0 = (brush.size.0 as isize + input.scroll_diff() as isize).max(1) as usize;
      }
      if !input.held_shift() {
        brush.size.1 = (brush.size.1 as isize + input.scroll_diff() as isize).max(1) as usize;
      }

      //Step simutation
      sim.step();

      //Request redraw
      window.request_redraw();
    }
  })
}
