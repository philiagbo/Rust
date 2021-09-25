
use std::sync:: {Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use super::particle_systems::ParticleSystem;

//gravity function for particles
pub fn gravity_thread(particle_list: Arc<Mutex<ParticleSystem>>, grav_bool: Arc<AtomicBool>) 
{
    loop
    {

        if grav_bool.load(Ordering::SeqCst)
        {
            //puts gravity thread in a lock so no other thread can gain access
            let guard_lock = particle_list.lock().unwrap(); 
            (*guard_lock).glob_dy.fetch_add(-6 as i32, Ordering::SeqCst);
        }
  
    std::thread::sleep(std::time::Duration::from_micros(2000));
    }  

}