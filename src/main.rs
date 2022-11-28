use ::rand::prelude::*;
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};


fn min(one: f32,two: f32) -> f32{
    match one < two {
        true => one,
        _ => two,
    }
} 
fn abs(num: f32) -> f32 {
    return num.powf(2.).sqrt()
} 

#[macroquad::main("map")]
async fn main() {
    let mut perlin = Perlin::new((0..1000).choose(&mut thread_rng()).unwrap());
    let mut secondary_perlin = Perlin::new((0..1000).choose(&mut thread_rng()).unwrap());
    let (mut base_x, mut base_y) = (0, 0);
    let mut zoom = 5.;
    let island_value = 4.;
    loop {
        clear_background(BLUE);
        key_presses(&mut perlin, &mut base_x, &mut base_y, &mut zoom);
        let (width, height) = (screen_width() as f64, screen_height() as f64);
        for x in 0..(width.round() as i32) {
            for y in 0..(height.round() as i32) {
                let (x_pos, y_pos) = (x + base_x, y + base_y);
                let xn = zoom * x_pos as f64 / width;
                let yn = zoom * y_pos as f64 / height;
                let d = min(1.,((xn as f32).powf(2.)+(yn as f32).powf(2.))/(2. as f32).sqrt()) * island_value;
                let val = perlin.get([xn, yn]) + ((1.-d)/2.) as f64 + 0.25 * secondary_perlin.get([xn*4., yn*4.]) + ((1.-d)/2.) as f64;
                if val > 0.7 {
                    draw_rectangle(x as f32, y as f32, 1., 1., LIGHTGRAY);
                } else if val > 0.2 {
                    draw_rectangle(x as f32, y as f32, 1., 1., GREEN);
                } else if val > 0.1 {
                    draw_rectangle(x as f32, y as f32, 1., 1., BEIGE);
                }
            }
        }
        next_frame().await
    }
}

fn key_presses(perlin: &mut Perlin, base_x: &mut i32, base_y: &mut i32, zoom: &mut f64) {
    if is_key_pressed(KeyCode::Space) {
        *perlin = Perlin::new((0..1000).choose(&mut thread_rng()).unwrap());
    }
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        *base_y -= 10;
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        *base_y += 10;
    }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        *base_x -= 10;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        *base_x += 10;
    }
    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Minus) {
        *zoom *= 2.;
    }
    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Equal) {
        *zoom /= 2.;
    } 
}
