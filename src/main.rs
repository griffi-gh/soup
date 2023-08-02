use winit::{
  window::WindowBuilder,
  dpi::LogicalSize,
  event_loop::EventLoop,
  event::{Event, WindowEvent},
};
use pixels::{Pixels, SurfaceTexture};

pub(crate) mod util;
pub(crate) mod particle;
pub(crate) mod simulation;
pub(crate) mod renderer;

use particle::{Particle, ParticleKind};
use simulation::Simulation;
use renderer::SimulationRenderer;

fn main() {
  let size = LogicalSize::new(Simulation::WIDTH as u32, Simulation::HEIGHT as u32);
  
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("soup")
    .with_inner_size(size)
    .with_min_inner_size(size)
    .build(&event_loop)
    .unwrap();

  let mut pixels = {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    Pixels::new(Simulation::WIDTH as u32, Simulation::HEIGHT as u32, surface_texture).unwrap()
  };
  
  let mut sim = Simulation::new();
  let mut ren = SimulationRenderer::new();

  for x in 300..=400 {
    for y in 100..=200  {
      *sim.get_mut((x, y)) = Particle::new(ParticleKind::Sand);
    }
  }

  for x in 100..700 {
    for y in 550..560  {
      *sim.get_mut((x, y)) = Particle::new(ParticleKind::Wall);
    }
  }

  event_loop.run(move |event, _, control_flow| {
    control_flow.set_poll();
    match event {
      Event::MainEventsCleared => {
        sim.step();
        window.request_redraw();
      },
      Event::RedrawRequested(_) => {
        ren.render(&sim);
        pixels.frame_mut().copy_from_slice(ren.buffer());
        pixels.render().unwrap();
      },
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => {
          control_flow.set_exit();
        },
        WindowEvent::Resized(new_size) => {
          pixels.resize_surface(new_size.width, new_size.height).unwrap();
        },
        _ => ()
      },
      _ => ()
    }
  })
}
