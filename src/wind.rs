use super::particle_systems::ParticleSystem;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
//wind function for particles
pub fn wind_thread(particle_list: Arc<Mutex<ParticleSystem>>, wind_bool: Arc<AtomicBool>, wind_direction: Arc<AtomicU16>) {

   loop
   {
       if wind_bool.load(Ordering::SeqCst)
        {
            //puts wind thread in a lock so no other thread can gain access
            let guard_lock = particle_list.lock().unwrap(); 
            let num_of_particles = (*guard_lock).list_of_particles.len();
            let wind_force = 3.0;
            let wind_dir = wind_direction.load(Ordering::SeqCst);
            

            for _i in 0..num_of_particles
            {
                //wind pushing upward
                if wind_dir == 0 
                {
                    
                    (*guard_lock).glob_dy.fetch_add(wind_force as i32, Ordering::SeqCst);

                }

                //wind pushing right
                if wind_dir == 1
                {
                    (*guard_lock).glob_dx.fetch_add(wind_force as i32, Ordering::SeqCst);

                }

                //wind pushing downward
                if wind_dir == 2
                {
                    (*guard_lock).glob_dy.fetch_add(-wind_force as i32, Ordering::SeqCst);

                }

                //wind pushing to the left
                if wind_dir == 3
                {
                    (*guard_lock).glob_dx.fetch_add(-wind_force as i32, Ordering::SeqCst);
                }

                //wind pushing to the forward
                if wind_dir == 4
                {
                    (*guard_lock).glob_dz.fetch_add(wind_force as i32, Ordering::SeqCst);
                
                }

                //wind pushing backwards
                if wind_dir == 5
                {
                    (*guard_lock).glob_dz.fetch_add(-wind_force as i32, Ordering::SeqCst);
                
                }
                
            
            }
            
        }

     std::thread::sleep(std::time::Duration::from_micros(1000));
    } 

}