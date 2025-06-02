#![warn(clippy::pedantic)]
use raylib::prelude::*;
use boids::types::{Vector2, Boid};

static BOIDS_COUNT: u32 = 2500;
static TURN_FACTOR: f32 = 0.5;
static VISUAL_RANGE: f32 = 40.0;
static PROTECTED_RANGE: f32 = 8.0;
static CENTERING_FACTOR: f32 = 0.0005;
static AVOID_FACTOR: f32 = 0.05;
static MATCHING_FACTOR: f32 = 0.05;
static MARGIN: f32 = 50.0;
static MIN_SPEED: f32 = 3.0;
static MAX_SPEED: f32 = 6.0;

fn main() {
    let (rl, thread) = raylib::init()
        .size(640,480)
        .title("Boids")
        .build();

    main_loop(rl, thread);
}

fn create_boids() -> Vec<Boid> {
    let mut boids: Vec<Boid> = Vec::new();
    let mut i = 0; 
    loop {
        if i >= BOIDS_COUNT { break; }

        // good enough
        let x: f32 = (rand::random::<f32>() % 100.0) + 200.0;
        let y: f32 = (rand::random::<f32>() % 100.0) + 200.0;

        boids.push(Boid {
            position: Vector2::new(x, y),
            velocity: Vector2::new(0.0, 0.0)
        });

        i += 1;
    }
    return boids;
}

fn main_loop(mut rl: RaylibHandle, thread: RaylibThread) {
    let mut boids = create_boids();

    // wont update automatically so if i make the window resizeable ill have to fix this
    let width = rl.get_screen_width();
    let height = rl.get_screen_height();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // iterate through every boid and do boid stuff with it
        let mut i = 0;
        loop {
            if i >= boids.len() { break }

            let mut boid_velocity;
            {
                let boid = boids.get(i).expect("uh oh");
                boid_velocity = boid.velocity;
                boid_velocity = boid_velocity + bounds(boid, width, height);
                boid_velocity = boid_velocity + movement(&boids, boid);

                let speed = boid_velocity.get_magnitude();
                if speed > MAX_SPEED {
                   boid_velocity = boid_velocity.divide_by_f32(speed).multiply_by_f32(MAX_SPEED);
                } else if speed < MIN_SPEED {
                    boid_velocity = boid_velocity.divide_by_f32(speed).multiply_by_f32(MIN_SPEED);
                }
            }

            let boid_mut = boids.get_mut(i).expect("WHAT?!");
            boid_mut.velocity = boid_velocity;
            boid_mut.position = boid_mut.position + boid_velocity;

            draw_boid(&mut d, boid_mut);

            i += 1;
        }
    }
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
fn draw_boid(d: &mut RaylibDrawHandle<'_>, boid: &mut Boid) {
    d.draw_circle(boid.position.x as i32, boid.position.y as i32, 1.0, Color::BLACK);
}