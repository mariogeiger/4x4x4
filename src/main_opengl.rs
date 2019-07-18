#[macro_use]
extern crate glium;

extern crate eventual;
extern crate negamax;
extern crate time;

mod cube;
mod glmath;
mod sphere;
mod state;

use negamax::GameState;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);

fn main() {
    // State of the game
    let mut state = state::State::new();
    let mut table = negamax::Table::new();

    use eventual::{Async, Future};
    use glium::Surface;
    use glmath::Mat4;

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new().with_title("4x4x4");
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let sphere = sphere::Sphere::new(&display, 30, 30);
    let cube = cube::Cube::new(&display);

    let board_verticies = vec![
        Vertex {
            position: [-2., -2., -2.],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [2., -2., -2.],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-2., 2., -2.],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [2., -2., -2.],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [2., 2., -2.],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-2., 2., -2.],
            normal: [0.0, 0.0, 1.0],
        },
    ];
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

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut theta: f32 = 3.14 / 4.0;
    let mut phi: f32 = 3.25 / 4.0;
    let mut scale: f32 = 1.0;

    let mut last_move = (4, 4, 4); // start out of bounds
    let mut mouse_last_pos = (0.0, 0.0);
    let mut mouse_pressed = false;
    let mut key_position = (0, 0);
    let mut player_turn = 1;

    let mut thread = None;

    loop {
        let mut target = display.draw();

        if state.win(1) {
            target.clear_color_and_depth((0.9, 0.0, 0.0, 1.0), 1.0);
        } else if state.win(-1) {
            target.clear_color_and_depth((0.9, 0.9, 0.0, 1.0), 1.0);
        } else if player_turn == 1 {
            target.clear_color_and_depth((0.1, 0.05, 0.05, 1.0), 1.0);
        } else if player_turn == -1 {
            target.clear_color_and_depth((0.1, 0.1, 0.05, 1.0), 1.0);
        } else {
            target.clear_color_and_depth((0.0, 0.1, 0.0, 1.0), 1.0);
        }

        let pers = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = width as f32 / height as f32;
            Mat4::perspective(aspect_ratio, 3.14f32 / 3.0, 0.1, 1024.0)
        };
        let view = Mat4::translation(0.0, 0.0, -8.0)
            * Mat4::rotation(theta, 1.0, 0.0, 0.0)
            * Mat4::rotation(phi, 0.0, 0.0, -1.0)
            * Mat4::scale(scale);

        let uniform = uniform! {
            model: Mat4::identity().0,
            view: view.0,
            perspective: pers.0,
            light: [0., 0., -3f32],
            high_color: [0.1, 0.6, 0.1f32],
            dark_color: [0.1, 0.3, 0.1f32]
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        target
            .draw(
                &board_verticies,
                &board_indices,
                &program,
                &uniform,
                &params,
            )
            .unwrap();

        for x in (0..4).rev() {
            for y in (0..4).rev() {
                for z in 0..4 {
                    let player = state.get(x, y, z);
                    let model = Mat4::translation(x as f32 - 1.5, y as f32 - 1.5, z as f32 - 1.5)
                        * Mat4::scale(if player == -1 { 0.4 } else { 0.49 });

                    let high_color;
                    let dark_color;
                    if player == 1 {
                        high_color = [1.0, 0.0, 0.0f32];
                        dark_color = [0.9, 0.1, 0.1f32];
                    } else if player == -1 {
                        high_color = [1.0, 1.0, 0.0f32];
                        dark_color = [0.9, 0.9, 0.1f32];
                    } else if player_turn == 1 && key_position.0 == x && key_position.1 == y {
                        high_color = [0.5, 0.5, 1.0f32];
                        dark_color = [0.5, 0.5, 0.9f32];
                    } else {
                        continue;
                    }

                    let light: [f32; 3] =
                        if last_move.0 == x && last_move.1 == y && last_move.2 == z {
                            let t = ((5.0 * time::precise_time_s()) % (2.0 * std::f64::consts::PI))
                                as f32;
                            [t.cos(), t.sin(), 0f32]
                        } else {
                            [0., 0., -3f32]
                        };

                    let uniform = uniform! {
                        model: model.0,
                        view: view.0,
                        perspective: pers.0,
                        light: light,
                        high_color: high_color,
                        dark_color: dark_color
                    };

                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            ..Default::default()
                        },
                        //polygon_mode: glium::draw_parameters::PolygonMode::Line,
                        backface_culling:
                            glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                        blend: glium::Blend::alpha_blending(),
                        ..Default::default()
                    };

                    if player == -1 {
                        target
                            .draw(
                                cube.get_positions(),
                                cube.get_indices(),
                                &program,
                                &uniform,
                                &params,
                            )
                            .unwrap();
                    } else {
                        target
                            .draw(
                                sphere.get_positions(),
                                sphere.get_indices(),
                                &program,
                                &uniform,
                                &params,
                            )
                            .unwrap();
                    }

                    if player == 0 {
                        break;
                    }
                }
            }
        }

        target.finish().unwrap();

        // AI turn
        if player_turn == -1 {
            // if thread not already running
            if thread.is_none() {
                let state = state.clone();
                let table = table.clone();

                thread = Some(Future::spawn(move || {
                    let mut state = state.clone();
                    let mut table = table.clone();

                    let mut best_value = -std::i32::MAX;
                    let mut alpha = -std::i32::MAX;
                    let beta = std::i32::MAX;

                    let t0 = time::precise_time_s();

                    let possibilities = state.possibilities(-1);
                    let n = possibilities.len();
                    let mut i = 0;
                    for y in possibilities {
                        println!("{}/{}...", i, n);
                        let v = -y.negamax_table(1, 6, -beta, -alpha, &mut table);
                        if v > best_value {
                            best_value = v;
                            state = y;
                        }
                        if v > alpha {
                            alpha = v;
                        }

                        i += 1;
                    }

                    table.clean();

                    let t1 = time::precise_time_s();

                    println!(
                        "value={} {:.2} seconds {} values into table",
                        best_value,
                        t1 - t0,
                        table.len()
                    );

                    (state, table)
                }));
            }

            // if thread finished
            if thread.as_ref().map_or(false, Future::is_ready) {
                let result = thread.unwrap().expect().unwrap();
                state = result.0;
                table = result.1;
                thread = None;
                player_turn = 1;
            }
        }

        use glium::glutin::{
            dpi, ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta, VirtualKeyCode,
            WindowEvent,
        };
        let mut stop = false;
        events_loop.poll_events(|ev| {
            if let Event::WindowEvent {
                window_id: _,
                event: ev,
            } = ev
            {
                match ev {
                    WindowEvent::CloseRequested => {
                        stop = true;
                    }

                    WindowEvent::KeyboardInput {
                        device_id: _,
                        input:
                            KeyboardInput {
                                scancode: _,
                                state: ElementState::Pressed,
                                virtual_keycode: Some(key_code),
                                modifiers: _,
                            },
                    } => match key_code {
                        VirtualKeyCode::Left => {
                            if key_position.0 > 0 {
                                key_position.0 -= 1;
                            }
                        }
                        VirtualKeyCode::Right => {
                            if key_position.0 < 3 {
                                key_position.0 += 1;
                            }
                        }
                        VirtualKeyCode::Down => {
                            if key_position.1 > 0 {
                                key_position.1 -= 1;
                            }
                        }
                        VirtualKeyCode::Up => {
                            if key_position.1 < 3 {
                                key_position.1 += 1;
                            }
                        }
                        VirtualKeyCode::Return | VirtualKeyCode::Space => {
                            if player_turn == 1 {
                                if state.add(key_position.0, key_position.1, player_turn) {
                                    player_turn = -player_turn;
                                }
                            }
                        }
                        VirtualKeyCode::Escape => {
                            if player_turn == 1 {
                                state = state::State::new();
                                last_move.0 = 4;
                            }
                        }
                        VirtualKeyCode::P => {
                            player_turn = -1;
                        }
                        _ => (),
                    },
                    WindowEvent::CursorMoved {
                        device_id: _,
                        position: dpi::LogicalPosition { x, y },
                        modifiers: _,
                    } => {
                        if mouse_pressed {
                            let dx = x - mouse_last_pos.0;
                            let dy = y - mouse_last_pos.1;

                            phi += (dx as f32) * 0.01;
                            theta -= (dy as f32) * 0.01;

                            phi = phi
                                .max(3.1416 * (-1.0 / 8.0))
                                .min(3.1416 * (1.0 / 2.0 + 1.0 / 8.0));
                            theta = theta.max(0.0).min(3.1416 * (1.0 / 2.0 + 1.0 / 8.0));
                        }
                        mouse_last_pos = (x, y);
                    }
                    WindowEvent::MouseInput {
                        device_id: _,
                        state,
                        button: MouseButton::Left,
                        modifiers: _,
                    } => match state {
                        ElementState::Pressed => {
                            mouse_pressed = true;
                        }
                        ElementState::Released => {
                            mouse_pressed = false;
                        }
                    },
                    WindowEvent::MouseWheel {
                        device_id: _,
                        delta: MouseScrollDelta::LineDelta(_, delta),
                        phase: _,
                        modifiers: _,
                    } => {
                        scale *= f32::powf(1.01, delta);
                    }
                    _ => (),
                }
            }
        });

        if stop {
            break;
        }
    }
}
