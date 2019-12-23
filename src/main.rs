use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::render::TextureAccess;

use std::time::Instant;
use std::time::Duration;

fn main() -> Result<(), String> {
	let context = sdl2::init()?;
	let video = context.video()?;

	let window = video.window("SDL2 test", 800, 600)
		.borderless()
		.resizable()
		.opengl()
		.build()
		.map_err(|e| e.to_string())?;
	let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
	let mut events = context.event_pump()?;

	let mut texture_creator = canvas.texture_creator();

	let mut application = Application::new(&mut events, &mut canvas, &mut texture_creator, 320, 240)?;
	application.run()?;
	Ok(())
}

struct Application<'a> {
	events: &'a mut sdl2::EventPump,
	canvas: &'a mut Canvas<Window>,
	texture: sdl2::render::Texture<'a>,
	width: u32,
	height: u32,
	should_quit: bool,
}

impl<'a> Application<'a> {
	fn new(
		events: &'a mut sdl2::EventPump,
		canvas: &'a mut Canvas<Window>,
		texture_creator: &'a mut sdl2::render::TextureCreator<sdl2::video::WindowContext>,
		width: u32,
		height: u32,
	) -> Result<Self, String> {
		let texture = texture_creator.create_texture(
			PixelFormatEnum::RGB888,
			TextureAccess::Streaming,
			width,
			height,
		).map_err(|e| e.to_string())?;

		Ok(Self {
			events,
			canvas,
			texture,
			width,
			height,
			should_quit: false,
		})
	}

	fn run(&mut self) -> Result<(), String> {
		let mut start = Instant::now();
		let mut count = 0;
		while !self.should_quit {
			while let Some(event) = self.events.poll_event() {
				match event {
					Event::Quit { .. } => self.should_quit = true,
					Event::KeyDown { keycode: Some(key), .. } => self.handle_key(key),
					_ => {},
				}
				if self.should_quit {
					return Ok(());
				}
			}

			self.draw_frame()?;
			self.canvas.present();

			count += 1;
			let elapsed = start.elapsed();
			if elapsed >= Duration::from_secs(1) {
				println!("Average FPS: {}", f64::from(count) / elapsed.as_secs_f64());
				count = 0;
				start = Instant::now();
			}
		}
		Ok(())
	}

	fn handle_key(&mut self, keycode: Keycode) {
		match keycode {
			Keycode::Escape => self.should_quit = true,
			_ => (),
		}
	}

	fn draw_frame(&mut self) -> Result<(), String> {
		let mut surface = self.canvas.window().surface(&self.events)?;
		let buf_width = self.width as usize;
		let buf_height = self.height as usize;

		self.texture.with_lock(None, |data, pitch| {
			for y in 0..buf_height {
				for x in 0..buf_width {
					let pixel = &mut data[pitch * y + x * 4..];
					pixel[0] = 0;
					pixel[1] = if x > y { 255 } else { 0 };
					pixel[2] = 0;
				}
			}
		});

		let buf_rect = Rect::new(0, 0, buf_width as u32, buf_height as u32);

		let buf_width  = f64::from(self.width);
		let buf_height = f64::from(self.height);
		let win_width  = f64::from(surface.width());
		let win_height = f64::from(surface.height());

		let ratio_w = win_width  / buf_width;
		let ratio_h = win_height / buf_height;
		let ratio = ratio_w.min(ratio_h);

		let x = (0.5 * (win_width  - buf_width  * ratio)).round() as i32;
		let y = (0.5 * (win_height - buf_height * ratio)).round() as i32;
		let win_rect = Rect::new(x, y, (buf_width * ratio) as u32, (buf_height * ratio) as u32);

		self.canvas.clear();
		self.canvas.copy(&self.texture, Some(buf_rect), Some(win_rect));

		Ok(())
	}
}
