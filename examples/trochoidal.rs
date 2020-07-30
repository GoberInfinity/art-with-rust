// https://en.wikipedia.org/wiki/Trochoidal_wave
use art::tools;
use std::f64::consts::E;
use std::iter;
use valora::prelude::rand::rngs::ThreadRng;
use valora::prelude::*;

// cargo run --example trochoidal -- -r 60
fn main() -> Result<()> {
    run_fn(Options::from_args(), |_, world, _| {
        //Outer circle
        let a: f32 = 60.;
        // Line
        let b: f32 = 2.;
        // Wave length
        let lambda = 100.;
        //  Wave number or repetency
        let k = (PI * 2.) / lambda;
        // Acceleration by gravity
        let g = 10.;
        // Phase speed with which the wave propagates in the x-direction.
        let c = (g / k).powf(0.5);
        let mut time: f32 = 0.;

        let calculate_x = |a: f32, b: f32, t: f32, k: f32, c: f32| {
            a + ((E as f32).powf(k * b)) / k * (k * (a + c * t)).sin()
        };
        let calculate_y = |a: f32, b: f32, t: f32, k: f32, c: f32| {
            b - ((E as f32).powf(k * b)) / k * (k * (a + c * t)).cos()
        };

        let total_rows = 20;

        let vertical_offset = world.height / (2. * total_rows as f32 + 1.);
        let center_point = world.center();
        let mut horizontal_movement = 1.;

        let mut rand_gen: ThreadRng = rand::thread_rng();
        let circle_colors: Vec<f32> = iter::repeat_with(|| {
            tools::math::generate_random_between(&mut rand_gen, 0, 100) as f32 * 0.01
        })
        .take(3)
        .collect();

        Ok(move |ctx: Context, canvas: &mut Canvas| {
            //Color of the background

            for col in -total_rows..=total_rows {
                canvas.set_color(LinSrgb::new(
                    circle_colors[0] * tools::colors::oscillate_using(&ctx, 0., 1.),
                    circle_colors[1] * tools::colors::oscillate_using(&ctx, 0., 1.),
                    circle_colors[2] * tools::colors::oscillate_using(&ctx, 0., 1.),
                ));
                let x = a as f32;
                let col = col as f32;
                let vertical_offset = vertical_offset * b * col as f32;
                canvas.paint(Filled(Ellipse::circle(
                    P2::new(
                        (calculate_x(x, b, time + x * col, k, c)
                            + center_point.x
                            + horizontal_movement)
                            % world.width,
                        calculate_y(x, b, time, k, c) + center_point.y + vertical_offset,
                    ),
                    1.,
                )));
            }

            time += 0.1;
            horizontal_movement += 0.5;
        })
    })
}
