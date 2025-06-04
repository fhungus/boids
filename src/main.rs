#![warn(clippy::pedantic)]
use std::thread::{self};

use raylib::prelude::*;
use boids::types::{Vector2, Boid, BoidDiff};


static THREADS: u8 = 8;

// some day i shall un-hardcode this
static WIDTH: i32 = 1000;
static HEIGHT: i32 = 1000;

static BOIDS_COUNT: u32 = 500; // per thread!
static TURN_FACTOR: f32 = 0.1;
static VISUAL_RANGE: f32 = 40.0;
static PROTECTED_RANGE: f32 = 8.0;
static CENTERING_FACTOR: f32 = 0.0005;
static AVOID_FACTOR: f32 = 0.05;
static MATCHING_FACTOR: f32 = 0.05;
static MARGIN: f32 = 50.0;
static MIN_SPEED: f32 = 3.0;
static MAX_SPEED: f32 = 6.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH,HEIGHT)
        .title("Boids: Cores & Cliffs Update")
        .build();

    let mut boids_vec = create_boids();

    let mut last_len = BOIDS_COUNT;
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        let mut next_boids_vec = Vec::new();
        for boids in boids_vec.iter() {
            let mut boids_diff = Vec::<Vec<BoidDiff>>::new();

            let mut threads = Vec::new();
            for other_boids in boids_vec.iter() {
                let boids_clone = boids.clone();
                let other_boids_clone = other_boids.clone();
                threads.push(thread::spawn(|| return calc_thread(boids_clone, other_boids_clone)));
            }

            for thread in threads {
                boids_diff.push(thread.join().unwrap());
            }

            let nextboids = add_boid_diffs(boids, boids_diff);
            next_boids_vec.push(nextboids);
        }

        // we missin
        let first = next_boids_vec.first().unwrap();
        if last_len != first.len() as u32 {
            let difference = last_len - first.len() as u32;
            println!("OH SHIT! We lost Jimmy... {difference}");
            last_len = first.len() as u32;
        }

        draw_boids(&mut d, &next_boids_vec);
        boids_vec = next_boids_vec;
    }
}

fn create_boids() -> Vec<Vec<Boid>> {
    let mut boids_array: Vec<Vec<Boid>> = Vec::new();
    
    let mut i = 0;
    loop {
        if i >= THREADS { break; }

        let mut boids: Vec<Boid> = Vec::new();

        let mut j = 0;
        loop {
            if j >= BOIDS_COUNT { break; }

            // good enough
            let x: f32 = (rand::random::<f32>() % 100.0) + 200.0;
            let y: f32 = (rand::random::<f32>() % 100.0) + 200.0;

            boids.push(Boid {
                position: Vector2::new(x, y),
                velocity: Vector2::new(0.0, 0.0)
            });

            j += 1;
        }

        boids_array.push(boids);
        i += 1;
    }

    return boids_array;
}

fn add_boid_diffs(base: &Vec<Boid>, diffs_vec: Vec<Vec<BoidDiff>>) -> Vec<Boid> {
    let mut results = Vec::new();

    let first = diffs_vec.get(0).expect("erm...");
    let mut i = 0;
    loop {
        if i >= first.len() as i64 {break;}

        // unneccesary redundancy?
        let base_boid_wrapped = base.get(i as usize);
        if base_boid_wrapped.is_none() {
            break
        }

        let mut base_boid = base_boid_wrapped.unwrap().clone(); 

        for diffs in diffs_vec.iter() {
            let diff = diffs.get(i as usize).unwrap();
            base_boid.velocity = base_boid.velocity + diff.velocity;
        } 

        // putting the speed test here because its the easiest place to do so
        let speed = base_boid.velocity.get_magnitude();
        if speed > MAX_SPEED {
            base_boid.velocity = base_boid.velocity.divide_by_f32(speed).multiply_by_f32(MAX_SPEED);
        } else if speed < MIN_SPEED {
            base_boid.velocity = base_boid.velocity.divide_by_f32(speed).multiply_by_f32(MIN_SPEED);
        }

        // and this is also the easiest place to move them...
        base_boid.position = base_boid.position + base_boid.velocity;

        results.push(base_boid);

        i += 1;
    }

    return results;
}

fn calc_thread(my_boids: Vec<Boid>, other_boids: Vec<Boid>) -> Vec<BoidDiff> {
    let mut result: Vec<BoidDiff> = Vec::new();

   let mut i: usize = 0;
   loop {
       if i as i64 >= my_boids.len() as i64 { break }

       let mut boid_velocity_diff;
       {
           let boid = my_boids.get(i).expect("uh oh");
           boid_velocity_diff = Vector2::new(0.0,0.0);
           boid_velocity_diff = boid_velocity_diff + bounds(boid, WIDTH, HEIGHT);
           boid_velocity_diff = boid_velocity_diff + movement(&other_boids, boid);
           
           let boid_diff = BoidDiff {
                velocity: boid_velocity_diff,
           };
           result.push(boid_diff);
       }

       i += 1;
   }

   return result;
}

fn bounds(boid: &Boid, w: i32, h: i32) -> Vector2 {
    let mut velocity = Vector2::new(0.0,0.0);

    if boid.position.x < MARGIN {
        velocity.x += TURN_FACTOR
    } else if boid.position.x > w as f32 - MARGIN {
        velocity.x -= TURN_FACTOR
    } else if boid.position.y < MARGIN {
        velocity.y += TURN_FACTOR
    } else if boid.position.y > h as f32 - MARGIN {
        velocity.y -= TURN_FACTOR
    }

    return velocity
}

fn movement(boids: &Vec<Boid>, boid: &Boid) -> Vector2 {
    let mut velocity = Vector2::new(0.0,0.0);

    let mut neighbours = 0;
    let mut close_v2 = Vector2::new(0.0, 0.0);
    let mut average_velocity = Vector2::new(0.0, 0.0);
    let mut centering_average = Vector2::new(0.0, 0.0);
    for i in boids.into_iter() {
        if boid.position == i.position { continue }; // we are NOT checking our own boid
        // SEPARATION
        if boid.position.in_range(i.position, PROTECTED_RANGE)  {
            close_v2 = close_v2 + (boid.position - i.position);
        }
        // ALIGNMENT / COHESION
        else if boid.position.in_range(i.position, VISUAL_RANGE) {
            average_velocity = average_velocity + i.velocity;
            centering_average = centering_average + i.position;
            neighbours += 1;
        }
    }

    velocity = velocity + close_v2.multiply_by_f32(AVOID_FACTOR);

    if neighbours > 0 {
        average_velocity.x = average_velocity.x / neighbours as f32;
        average_velocity.y = average_velocity.y / neighbours as f32;

        centering_average.x = centering_average.x / neighbours as f32;
        centering_average.y = centering_average.y / neighbours as f32;
    }
    velocity = velocity + average_velocity.multiply_by_f32(MATCHING_FACTOR);
    velocity = velocity + (centering_average - boid.position).multiply_by_f32(CENTERING_FACTOR);

    return velocity;
}

// uh, what does <'_> mean?!?!
fn draw_boids(d: &mut RaylibDrawHandle<'_>, boids_vec: &Vec<Vec<Boid>>) {
    for boids in boids_vec.iter() {
        for boid in boids.iter() {
            d.draw_circle(boid.position.x as i32, boid.position.y as i32, 1.5, Color::BLACK);
        }
    }
}