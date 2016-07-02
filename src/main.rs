#[macro_use]
extern crate glium;

extern crate eventual;

mod sphere;
mod state;
mod glmath;

#[derive(Clone, Copy)]
struct Vertex {
	position: [f32; 3],
	normal: [f32; 3]
}

implement_vertex!(Vertex, position, normal);

fn main() {
	use glium::{DisplayBuild, Surface};
	use glmath::Mat4;
   	use eventual::{Future, Async};

	let display = glium::glutin::WindowBuilder::new()
			.with_depth_buffer(24)
			.build_glium().unwrap();
			
	let sphere = sphere::Sphere::new(&display, 30, 30);
	
	let board_verticies = vec![
		Vertex{ position: [-2., -2., -2.], normal: [0.0, 0.0, 1.0] },
		Vertex{ position: [ 2., -2., -2.], normal: [0.0, 0.0, 1.0] },
		Vertex{ position: [-2.,  2., -2.], normal: [0.0, 0.0, 1.0] },
		Vertex{ position: [ 2., -2., -2.], normal: [0.0, 0.0, 1.0] },
		Vertex{ position: [ 2.,  2., -2.], normal: [0.0, 0.0, 1.0] },
		Vertex{ position: [-2.,  2., -2.], normal: [0.0, 0.0, 1.0] }];
	let board_verticies = glium::VertexBuffer::new(&display, &board_verticies).unwrap();
	let board_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	let vertex_shader_src = r#"
		#version 150
		
		in vec3 position;
		in vec3 normal;
		
		uniform mat4 model;
		uniform mat4 view;
		uniform mat4 perspective;
		uniform vec3 light;
		
		smooth out vec3 l;
		smooth out vec3 n;
		smooth out vec3 p;
		
		void main() {
			mat4 modelview = view * model;
			l = transpose(inverse(mat3(view))) * light;
			n = transpose(inverse(mat3(modelview))) * normal;
			p = position;
			gl_Position = perspective * modelview * vec4(position, 1.0);
		}
	"#;
	let fragment_shader_src = r#"
		#version 150
		
		uniform vec3 dark_color;
		uniform vec3 high_color;
		
		smooth in vec3 l;
		smooth in vec3 n;
		smooth in vec3 p;
		
		out vec4 color;

		void main() {
			vec3 nl = normalize(l);
			vec3 nn = normalize(n);
			vec3 nr = nl - 2 * nn * dot(nl,nn);
			float brightness = clamp(0, -dot(nn, nl), 1);
			float specular = pow(max(nr.z, 0), 16);
			vec3 hc = high_color;
			vec3 dc = dark_color;
			color = vec4(mix(dc, hc, brightness) + specular * vec3(1,1,1), 0.95);
		}
	"#;

	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

	let mut theta: f32 = 3.14 / 4.0;
	let mut phi: f32 = 3.25 / 4.0;
	let mut scale: f32 = 1.0;

	let mut mouse_last_pos = (0,0);
	let mut mouse_pressed = false;
	let mut key_position = (0,0);
	let mut player_turn = 0;
	
	let mut state = state::State::new();

	let mut thread = Vec::new();
	
	loop {
		let mut target = display.draw();
		
		if state.win(0) {
			target.clear_color_and_depth((0.9, 0.0, 0.0, 1.0), 1.0);
		} else if state.win(1) {
			target.clear_color_and_depth((0.9, 0.9, 0.0, 1.0), 1.0);
		} else {
			target.clear_color_and_depth((0.0, 0.1, 0.0, 1.0), 1.0);
		}
		
		let pers = {
			let (width, height) = target.get_dimensions();
			let aspect_ratio = width as f32 / height as f32;
			Mat4::perspective(aspect_ratio, 3.14f32 / 4.0, 0.1, 1024.0)
		};
		let view = Mat4::translation(0.0, 0.0, -8.0) 
			* Mat4::rotation(theta, 1.0, 0.0, 0.0) * Mat4::rotation(phi, 0.0, 0.0, -1.0) 
			* Mat4::scale(scale);
		
		
		let uniform = uniform!{
			model: Mat4::identity().0,
			view: view.0,
			perspective: pers.0,
			light: [1., -1., -1f32],
			high_color: [0.1, 0.6, 0.1f32],
			dark_color: [0.1, 0.3, 0.1f32]
		};
		
		let params = glium::DrawParameters {
			depth: glium::Depth {
				test: glium::draw_parameters::DepthTest::IfLess,
				write: true,
				.. Default::default()
			},
			blend: glium::Blend::alpha_blending(),
			.. Default::default()
		};

		
		target.draw(&board_verticies, &board_indices, &program, &uniform, &params).unwrap();
			
		for x in (0..4).rev() {
		for y in (0..4).rev() {
		for z in 0..4 {
			let model = Mat4::translation(x as f32 - 1.5, y as f32 - 1.5, z as f32 - 1.5)
				* Mat4::scale(0.45);
			let player = state.get(x, y, z);
		
			let high_color;
			let dark_color;
			if player == 0 {
				high_color = [1.0, 0.0, 0.0f32];
				dark_color = [0.9, 0.1, 0.1f32];				
			} else if player == 1 {
				high_color = [1.0, 1.0, 0.0f32];
				dark_color = [0.9, 0.9, 0.1f32];				
			} else if key_position.0 == x && key_position.1 == y {
				high_color = [0.5, 0.5, 1.0f32];
				dark_color = [0.5, 0.5, 0.9f32];				
			} else {
				continue;
			}
		
			let uniform = uniform!{
				model: model.0,
				view: view.0,
				perspective: pers.0,
				light: [1., -1., -1f32],
				high_color: high_color,
				dark_color: dark_color
			};
		
			let params = glium::DrawParameters {
				depth: glium::Depth {
					test: glium::draw_parameters::DepthTest::IfLess,
					write: true,
					.. Default::default()
				},
				//polygon_mode: glium::draw_parameters::PolygonMode::Line,
				backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
				blend: glium::Blend::alpha_blending(),
				.. Default::default()
			};
	
			target.draw(sphere.get_positions(), sphere.get_indices(), &program, &uniform, &params).unwrap();
			
			if player == -1 { break; }
		}
		}
		}

		target.finish().unwrap();
		
		if player_turn == 1 {
			if thread.is_empty() {
				let state = state.clone();
				
				thread.push(Future::spawn(move || {
					let mut best_value = -std::i32::MAX;
					let mut alpha = -std::i32::MAX;
					let beta = std::i32::MAX;
					let mut best_mov = (0,0);

					for mov in state.possibilities() {
						let mut y = state.clone();
						y.add(mov.0, mov.1, 1);
						let v = -y.negamax(0, 6, -beta, -alpha);
						if v > best_value {
							best_value = v;
							best_mov = mov;
						}
						if v > alpha { alpha = v; }
					}
					println!("{}{} value = {}", best_mov.0 + 1, best_mov.1 + 1, best_value);
					
					let mut state = state.clone();
					state.add(best_mov.0, best_mov.1, 1);
					state
				}));
			} else if thread[0].is_ready() {
				state = thread.pop().unwrap().expect().unwrap();
				player_turn = 1 - player_turn;
			}
		}

		for ev in display.poll_events() {
			use glium::glutin::*;
			match ev {
				Event::Closed => return,
				Event::KeyboardInput(key_state, value, key_code) => {
					println!("{:?} {:?} {:?}", key_state, value, key_code);
					if key_state == ElementState::Pressed && key_code.is_some() {
						let key_code = key_code.unwrap();
						match key_code {
							VirtualKeyCode::Left => {
								if key_position.0 > 0 { key_position.0 -= 1; }
							}
							VirtualKeyCode::Right => {
								if key_position.0 < 3 { key_position.0 += 1; }
							}
							VirtualKeyCode::Down => {
								if key_position.1 > 0 { key_position.1 -= 1; }
							}
							VirtualKeyCode::Up => {
								if key_position.1 < 3 { key_position.1 += 1; }
							}
							VirtualKeyCode::Return|VirtualKeyCode::Space => {
								if state.add(key_position.0, key_position.1, player_turn) {
									player_turn = 1 - player_turn;
								}
							}
							VirtualKeyCode::Escape => {
								state = state::State::new();
							}
							_ => ()
						}
					}
				},
				Event::MouseMoved(x, y) => {
					if mouse_pressed {
						let dx = x - mouse_last_pos.0;
						let dy = y - mouse_last_pos.1;
						
						phi += (dx as f32) * 0.01;
						theta -= (dy as f32) * 0.01;
						
						phi = phi.max(3.1416 * (-1.0/8.0)).min(3.1416 * (1.0/2.0 + 1.0/8.0));
						theta = theta.max(0.0).min(3.1416 * (1.0/2.0 + 1.0/8.0));
					}
					mouse_last_pos = (x,y);
				},
				Event::MouseInput(state, button) => {
					if button == MouseButton::Left {
						match state {
							ElementState::Pressed => { mouse_pressed = true; }
							ElementState::Released => { mouse_pressed =  false; }
						}
					}
				},
				Event::MouseWheel(MouseScrollDelta::LineDelta(_, delta), _) => {
					scale *= f32::powf(1.01, delta);
				},
				_ => ()
			}
		}
	}
}
