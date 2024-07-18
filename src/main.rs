mod bmp;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
use framebuffer::Framebuffer;

fn render(framebuffer: &mut Framebuffer) {

  let mut patterns = vec![
      // Glider
      vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
      // Blinker
      vec![(10, 10), (11, 10), (12, 10)],
      // Toad
      vec![(20, 20), (21, 20), (22, 20), (21, 21), (22, 21), (23, 21)],
      // LWSS
      vec![
          (30, 30), (31, 30), (32, 30), (33, 30), (30, 31), (34, 31), (34, 32), (33, 33),
          (30, 33),
      ],
      // Beacon
      vec![(40, 40), (41, 40), (40, 41), (43, 42), (42, 43), (43, 43)],
      // Pulsar
      vec![
          (50, 48), (51, 48), (52, 48), (56, 48), (57, 48), (58, 48), (48, 50), (53, 50),
          (55, 50), (60, 50), (48, 51), (53, 51), (55, 51), (60, 51), (48, 52), (53, 52),
          (55, 52), (60, 52), (50, 53), (51, 53), (52, 53), (56, 53), (57, 53), (58, 53),
          (50, 55), (51, 55), (52, 55), (56, 55), (57, 55), (58, 55), (48, 56), (53, 56),
          (55, 56), (60, 56), (48, 57), (53, 57), (55, 57), (60, 57), (48, 58), (53, 58),
          (55, 58), (60, 58), (50, 60), (51, 60), (52, 60), (56, 60), (57, 60), (58, 60),
      ],
      // Beehive
      vec![(60, 10), (61, 10), (59, 11), (62, 11), (60, 12), (61, 12)],
      // Loaf
      vec![(70, 20), (71, 20), (69, 21), (72, 21), (70, 22), (72, 22), (71, 23)],
      // Tub
      vec![(80, 30), (81, 30), (79, 31), (81, 31), (80, 32)],
      // Penta-decathlon
      vec![
          (20, 50), (21, 50), (22, 50), (23, 50), (24, 50), (25, 50), (26, 50), (27, 50),
          (20, 52), (21, 52), (22, 52), (23, 52), (24, 52), (25, 52), (26, 52), (27, 52),
          (22, 54), (23, 54), (24, 54), (25, 54),
      ],
  ];

  // Rellenar la pantalla con múltiples instancias de los patrones
  for y_offset in (0..framebuffer.height).step_by(20) {
      for x_offset in (0..framebuffer.width).step_by(20) {
          for pattern in &patterns {
              for &(x, y) in pattern {
                  framebuffer.point((x + x_offset) % framebuffer.width, (y + y_offset) % framebuffer.height);
              }
          }
      }
  }
}

fn main() {
  let window_width = 800;
  let window_height = 600;

  let framebuffer_width = 100;
  let framebuffer_height = 100;

  let frame_delay = Duration::from_millis(100);

  let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

  // Inicializar el framebuffer con el patrón inicial
  render(&mut framebuffer);

  let mut window = Window::new(
      "Conway's Game of Life",
      window_width,
      window_height,
      WindowOptions::default(),
  )
  .unwrap();

  while window.is_open() {
      // Listen to inputs
      if window.is_key_down(Key::Escape) {
          break;
      }

      // Actualizar el estado del framebuffer
      framebuffer.buffer = framebuffer.next_state();

      // Update the window with the framebuffer contents
      window
          .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
          .unwrap();

      std::thread::sleep(frame_delay);
  }
}