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
pub(crate) mod heat;
pub(crate) mod renderer;
pub(crate) mod brush;

use particle::{Particle, Element};
use simulation::Simulation;
use heat::HeatSimulation;
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
  let mut heat_sim = HeatSimulation::new();
  let mut renderer = SimulationRenderer::new();

  for x in 50..(Simulation::IWIDTH - 50) {
    for y in (Simulation::IHEIGHT - 50)..(Simulation::IHEIGHT - 40)  {
      *sim.get_mut((x, y)) = Particle::spawn(Element::Wall);
    }
  }

  let mut brush = Brush::default();
  let mut selection = Element::Sand;

  event_loop.run(move |event, _, control_flow| {
    control_flow.set_poll();

    if let Event::RedrawRequested(_) = event {
      //render simulation
      renderer.render(&sim);
      pixels.frame_mut().copy_from_slice(renderer.buffer());

      //render brush
      for pos in brush.centered().iter() {
        if !Simulation::fits(pos) { continue }
        let base = 4 * (Simulation::WIDTH * pos.1 as usize + pos.0 as usize);
        let pixel = &mut pixels.frame_mut()[base..(base + 4)];
        for p in pixel.iter_mut().take(3) {
          *p = p.saturating_add(64);
        }
        pixel[3] = 0xff;
      }

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
        (VirtualKeyCode::Key4, Element::Fire),
      ];
      for (key, elem) in KEY_ELEMENT_MAP {
        if input.key_pressed(*key) {
          selection = *elem;
        }
      }

      // Handle brush/mouse
      if let Some(mouse) = input.mouse() {
        brush.position = (mouse.0 as i32, mouse.1 as i32);
      }
      if input.mouse_held(0) || input.mouse_held(1) {
        let elem = if input.mouse_held(0) { selection } else { Element::Air };
        for pos in brush.centered().iter() {
          if !Simulation::fits(pos) { continue }
          let particle = sim.get_mut(pos);
          if particle.element == Element::Air || elem == Element::Air {
            *particle = Particle::spawn(elem);
          }
        }
      }
      let size_input =
        input.scroll_diff()
        + (5. * input.key_pressed(VirtualKeyCode::RBracket) as u32 as f32)
        - (5. * input.key_pressed(VirtualKeyCode::LBracket) as u32 as f32);
      if !input.held_control() {
        brush.size.0 = (brush.size.0 as i32 + size_input as i32).max(1) as u32;
      }
      if !input.held_shift() {
        brush.size.1 = (brush.size.1 as i32 + size_input as i32).max(1) as u32;
      }

      //Step simutation
      sim.step();
      heat_sim.step(&mut sim);

      //Request redraw
      window.request_redraw();
    }
  })
}
