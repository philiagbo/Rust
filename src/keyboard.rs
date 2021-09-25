use device_query::{DeviceQuery, DeviceState, Keycode};

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};

pub fn keyboard_thread(grav_bool: Arc<AtomicBool>, wind_bool: Arc<AtomicBool>, color: Arc<AtomicU16>, wind_direction: Arc<AtomicU16>)
{
    loop
    {
         //get current device state
        let device_state = DeviceState::new(); 
        let current_key : Vec<Keycode> = device_state.get_keys();

        //check key pressed for grav_bool
        if current_key.contains(&Keycode::G) 
        { 
         
            grav_bool.store(true, Ordering::SeqCst);
        }
        else
        {
            grav_bool.store(false, Ordering::SeqCst);
        }

        //check W key pressed for wind_bool
        if current_key.contains(&Keycode::W) 
        { 
            wind_bool.store(true, Ordering::SeqCst);
        }
        else
        {
            wind_bool.store(false, Ordering::SeqCst);
        }

        //checking keys for colors
        if current_key.contains(&Keycode::Key1)
        {
            //solid colorr
            color.store(0, Ordering::SeqCst);

            //brightness based on speed
            //color.store(1, Ordering::SeqCst);

            //brightness based on distance from other molecules
            //color.store(2, Ordering::SeqCst);
        }
        else if current_key.contains(&Keycode::Key2)
        {
            color.store(1, Ordering::SeqCst);
        }
        else if current_key.contains(&Keycode::Key3)
        {
            color.store(2, Ordering::SeqCst);
        }

    
        if current_key.contains(&Keycode::Space) 
        {
            let mut wind_controller = wind_direction.load(Ordering::SeqCst);
            wind_controller +=1;
            if wind_controller > 5 
            {
                wind_controller = 0;
                
            }
            wind_direction.store(wind_controller, Ordering::SeqCst);

        }
        std::thread::sleep(std::time::Duration::from_micros(2000));
    }
   
    

}