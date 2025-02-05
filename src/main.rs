use glfw::{Action, Context, Key, WindowEvent};
//use gl::types::*;

mod shader;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    let (mut window, events) = glfw.create_window(800, 600, "42run", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

	let shader = shader::Shader::new(
        "shaders/vertex/basic.glsl",
        "shaders/fragment/basic.glsl"
    ).expect("Failed to load shaders");

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
			gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            // Activate shader
            shader.use_program();
            
            // Set up transformation matrices
            let model = nalgebra::Matrix4::identity();
            let view = nalgebra::Matrix4::look_at_rh(
                &nalgebra::Point3::new(0.0, 0.0, 3.0),
                &nalgebra::Point3::origin(),
                &nalgebra::Vector3::y()
            );
            let projection = nalgebra::Perspective3::new(
                800.0 / 600.0,
                45.0f32.to_radians(),
                0.1,
                100.0
            ).to_homogeneous();
            
            // Set uniforms
            shader.set_mat4("model", &model);
            shader.set_mat4("view", &view);
            shader.set_mat4("projection", &projection);
            
            // Render meshes here
        }

        window.swap_buffers();
		glfw.poll_events();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape,  _, Action::Press, _) => window.set_should_close(true),
		glfw::WindowEvent::Key(Key::Q,  _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}