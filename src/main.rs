#[macro_use]
extern crate glium;
mod particle_systems;
mod gravity;
mod wind;
mod visualisation;
mod keyboard;

use particle_systems::Particle;
use particle_systems::ParticleSystem;

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU16, AtomicBool,};



fn main() 
{
    //initialising all Arcs that will be shared accross threads 
    let wind_bool: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let grav_bool: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let color: Arc<AtomicU16> = Arc::new(AtomicU16::new(0));
    let wind_direction: Arc<AtomicU16> = Arc::new(AtomicU16::new(0));

    let mut particle_system: ParticleSystem = ParticleSystem::new();
    

    //particle_system.list_of_particles.push(temp);

   // particle_system.init_particle();

    //defining the Arc of a mutex to allow one thread access to the particle system
    let p_s_a: Arc<Mutex<ParticleSystem>> = Arc::new(Mutex::new(particle_system));
    

    //Threads for Partical system
    let p_s_a_clone = p_s_a.clone();
    let second_p_s_a_clone = p_s_a.clone();
    std::thread::spawn(move || particle_systems::particle_system_thread(p_s_a_clone));
    std::thread::spawn(move || particle_systems::particle_system_thread(second_p_s_a_clone));


    //keyboard Thread
    let grav_bool_for_keyboard = grav_bool.clone();
    let wind_bool_for_keyboard = wind_bool.clone();
    let color_for_keyboard = color.clone();
    let wind_direction_for_keyboard = wind_direction.clone();
    std::thread::spawn(move || keyboard::keyboard_thread(grav_bool_for_keyboard, wind_bool_for_keyboard, color_for_keyboard, wind_direction_for_keyboard));
    //Threads for Gravity
    let grav_bool_gravity = grav_bool.clone();
    let parcticles_for_gravity = p_s_a.clone();
    std::thread::spawn(move || gravity::gravity_thread(parcticles_for_gravity, grav_bool_gravity));

    //Thread for Wind
    let particles_for_wind = p_s_a.clone();
    let wind_bool_for_wind = wind_bool.clone();
    let wind_direction_for_wind = wind_direction.clone();
    std::thread::spawn(move || wind::wind_thread(particles_for_wind, wind_bool_for_wind, wind_direction_for_wind));

    //Visualisation
    let particles_for_vis = p_s_a.clone();
    let color_for_vis = color.clone();

    visualisation::visualisation(particles_for_vis, color_for_vis);

}