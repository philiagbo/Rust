use super:: particle_systems::Particle;
use super:: particle_systems::ParticleSystem;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU16, Ordering};


pub fn visualisation(particle_list: Arc<Mutex<ParticleSystem>>, color: Arc<AtomicU16>) {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
    }
    implement_vertex!(Vertex, position);
    let vertex1 = Vertex { position: [-0.05, -0.0288, 1.0] };
    let vertex2 = Vertex { position: [ 0.00,  0.0577, 1.0] };
    let vertex3 = Vertex { position: [ 0.05, -0.0288, 1.0] };
    let shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        uniform mat4 matrix;
        uniform mat4 perspective;  
        void main() {
            gl_Position = perspective * matrix * vec4(position, 1.0);
        }
    "#;
    // let fragment_shader_src = r#"
    //     #version 140
    //     out vec4 color;
    //     void main() {
    //         color = vec4(1.0, 0.0, 0.0, 1.0);
    //     }
    // "#;
    
    
    event_loop.run(move |event, _, control_flow| {
       
       
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        // Begin render loop
        // Get a drawing canvas
        let mut target = display.draw();
        // Clear the screen
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let current_time = std::time::Instant::now();

        {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;
            let fov: f32 = 3.141592 / 3.0; // Field of view
            let zfar = 1024.0;  // Far clipping plain
            let znear = 0.1; // Near clipping plain
            let f = 1.0 / (fov / 2.0).tan();
            // Loop through each of 10 triangles

            let mut guard_lock = particle_list.lock().unwrap();
            let num_of_particles = (*guard_lock).list_of_particles.len();
           
            for i in 0 .. num_of_particles {
                let mut particle_speed: f32 = (*guard_lock).get_and_apply_speed(i);
                // Calculate the x and y position of a triangle
                let mut p: Particle = (*guard_lock).list_of_particles[i as usize];
                for j in i+1 ..num_of_particles
                {
                    if j !=i
                    {
                        //let second_p = (*guard_lock).list_of_particles[j];
                        (*guard_lock).particle_mutation(i, j);
                    }
                } 
                

                println!("Particle {}: {}, {}", i, p.x, p.y);
                let pos_x : f32 = (p.x / 100.0) * 0.71;//(p.x / 50.0) - 1.0;
                let pos_y : f32 = (p.y / 100.0)* 0.52; //(p.y / 50.0) - 1.0;
                let pos_z : f32 = (p.z / 100.0) * 0.5;  // 100.0;
                
                // Create a 4x4 matrix to hold the position and orientation of the triangle
                // and a 4x4 matrix to hold the camera perspective correction
                let uniforms = uniform! {
                    matrix: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [pos_x, pos_y, pos_z, 1.0],
                    ],
                    perspective: [
                        [f*aspect_ratio, 0.0, 0.0, 0.0],
                        [0.0, f, 0.0, 0.0],
                        [0.0, 0.0, (zfar+znear)/(zfar-znear), 1.0],
                        [0.0, 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
                    ]
                };

                let mut temp_color_change_src: String = "".to_owned();

                let current_color = color.load(Ordering::SeqCst);
            
                if current_color == 0
                {
                    temp_color_change_src = change_color(1.0, 0.0, 0.0);
                }
                else if current_color == 1 // speed mode
                {
                    if particle_speed > 7.0
                    {
                        particle_speed = 7.0;
                    }
                    temp_color_change_src = change_color(0.0, particle_speed/7.0, 0.0);
                }
                else if current_color == 2
                {
                    let mut closest_particle_distance = 100.0;
                    for j in 0..num_of_particles
                    {
                        if j != i
                        {
                            closest_particle_distance = p.get_distance_to_nearest_particle((*guard_lock).list_of_particles[j], closest_particle_distance);
                            println!("{}", closest_particle_distance);
                        }
                    }
                    
                    if closest_particle_distance > 90.0 { closest_particle_distance = 90.0; }
                    let brightness = 1.0 - (closest_particle_distance / 100.0);
                    temp_color_change_src = change_color(0.0, 0.0, brightness);
                }
                

                let program = glium::Program::from_source(&display, vertex_shader_src, &temp_color_change_src, None).unwrap();

                // Draw the triangle
                //println!("draw triangle");
                target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
            }
            // Calculate the data for the camera configuration
            
            (*guard_lock).glob_dx.store(0, Ordering::SeqCst);
            (*guard_lock).glob_dy.store(0, Ordering::SeqCst);
            (*guard_lock).glob_dz.store(0, Ordering::SeqCst);

        
        }
        // Display the new image
        target.finish().unwrap();

        println!("Time taken: {}ms", current_time.elapsed().as_millis());
        // End render loop

    });


} 

fn change_color(r:f32, g:f32, b:f32) -> String 
{
    format!(r#"
    #version 140
    out vec4 color;
    void main() {{
       color = vec4({}, {}, {}, 1.0);
    }}
"#, r, g, b)
}