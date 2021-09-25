use std::sync:: {Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering,};

//setting the boundary of the cubic space 

#[derive (Debug, Copy, Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub dx: f32,
    pub dy: f32,
    pub dz: f32,
    
}

impl Particle {
    //setting the initial coordinates and radius for each particle
    pub fn new() -> Particle {
        Particle{
            x:(rand::random::<f32>() * 10.0) + 2.0,
            y:(rand::random::<f32>() * 10.0) + 2.0,
            z:rand::random::<f32>(),
            dx:0.0,
            dy:0.0,
            dz:0.0,
        }
    }
    //moves each particle a random distance within the enclosure
    pub fn brownian_motion(&mut self)
    {
            let limit: f32 = 100.0;
            self.dx +=  (rand::random::<f32>() * 2.0) - 1.0;
            if self.x + self.dx < -limit {
                self.dx += limit * 2.0;
            } 
            else if self.x + self.dx >= limit {
                self.dx -= limit * 2.0;
            }

            self.dy +=  (rand::random::<f32>() * 2.0) - 1.0;
            if self.y + self.dy < -limit {
                self.dy += limit * 2.0;
            } 
            else if self.y + self.dy >= limit {
                self.dy -= limit * 2.0;
            }

            self.z += (rand::random::<f32>() * 2.0) - 1.0;
            if self.z + self.dz < -limit{
                self.z += limit * 2.0;
            }
            else if self.z + self.dz >= limit {
                self.z-= limit * 2.0;
            }
  
    }

    pub fn get_distance_to_nearest_particle(&mut self, particle: Particle, closest_distance: f32) -> f32
    {
        let distance_between_particles = (self.x - particle.x).abs() + (self.y - particle.y).abs() + (self.z - particle.z).abs();
        
        if distance_between_particles < closest_distance { return distance_between_particles }
 
        return closest_distance
    }

    pub fn collide(& self, particle: Particle) -> bool{
        if (self.x - particle.x <= 0.0001) && (self.y - particle.y <= 0.0001) && (self.z - particle.z <= 0.0001){
            return true;
        }
        else {
            return false;
        }
    }


}

//Partical system struct
pub struct ParticleSystem {
    pub size_x: f32,
    pub size_y: f32,
    pub size_z: f32,
    pub glob_dx: AtomicI32,
    pub glob_dy: AtomicI32,
    pub glob_dz: AtomicI32,
    pub list_of_particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> ParticleSystem {

        ParticleSystem {
            size_x: 100.0,
            size_y: 100.0,
            size_z: 100.0,
            glob_dx: AtomicI32::new(0),
            glob_dy: AtomicI32::new(0),
            glob_dz: AtomicI32::new(0),
            list_of_particles:vec![Particle::new(); 5], 
        } 
    }
    

    pub fn get_and_apply_speed(&mut self, index: usize)-> f32
    {
        self.list_of_particles[index].x += self.list_of_particles[index].dx + self.glob_dx.load(Ordering::SeqCst) as f32;
        self.list_of_particles[index].y += self.list_of_particles[index].dy + self.glob_dy.load(Ordering::SeqCst) as f32;
        self.list_of_particles[index].z += self.list_of_particles[index].dz + self.glob_dz.load(Ordering::SeqCst) as f32;

        let speed = self.list_of_particles[index].dx.abs() + self.list_of_particles[index].dy.abs() + self.list_of_particles[index].dz.abs();

        self.list_of_particles[index].dx = 0.0;
        self.list_of_particles[index].dy = 0.0;
        self.list_of_particles[index].dz = 0.0;

        speed   

    }

    pub fn particle_mutation(&mut self,  particle: usize, particle_two: usize)
    {
        let p1 = self.list_of_particles[particle];
        let p2 = self.list_of_particles[particle_two];

        if p1.collide(p2) == true
        {
            //setting random coordinates for mutated particles
            let mutation_particle_x = (rand::random::<f32>() * 2.0) - 1.0;
            let mutation_particle_y = (rand::random::<f32>() * 2.0) - 1.0;
            let mutation_particle_z = (rand::random::<f32>() * 2.0) - 1.0;

            self.list_of_particles[particle].x += 10.0;
            self.list_of_particles[particle].y += 10.0;

            self.list_of_particles[particle_two].x  -=10.0;
            self.list_of_particles[particle_two].y -=10.0;
            


            //create new particle
            let mut new_particle = Particle::new();

            let spawn_buffer: f32 = 50.0;
            //setting new particles coordianates to be random floats
            new_particle.x = self.list_of_particles[particle].x + mutation_particle_x + spawn_buffer;
            new_particle.y = self.list_of_particles[particle].y + mutation_particle_y + spawn_buffer;
            new_particle.z =  self.list_of_particles[particle].z + mutation_particle_z + spawn_buffer;


            let mut new_particle2 = Particle::new();
            //setting new particles coordianates to be random floats
            new_particle2.x = self.list_of_particles[particle_two].x - mutation_particle_x - spawn_buffer;
            new_particle2.y = self.list_of_particles[particle_two].y - mutation_particle_y - spawn_buffer;
            new_particle2.z =  self.list_of_particles[particle_two].z - mutation_particle_z - spawn_buffer;
            
        
            //pushing mutated particle into the list 
            self.list_of_particles.push(new_particle);
            self.list_of_particles.push(new_particle2);

        }
        
    }

}


pub fn particle_system_thread(particle_list: Arc<Mutex<ParticleSystem>>)
{
    loop 
    {
        let mut guard_lock = particle_list.lock().unwrap(); 
        let num_of_particles = (*guard_lock).list_of_particles.len();

        for i in 0..num_of_particles
        {
            (*guard_lock).list_of_particles[i].brownian_motion();
            for j in 1..num_of_particles
            {
                if j != i
                {
                    //(*guard_lock).list_of_particles[i].collide((*guard_lock).list_of_particles[j]);
                }              
            }          
        }
        std::thread::sleep(std::time::Duration::from_micros(500));
    } 
}
