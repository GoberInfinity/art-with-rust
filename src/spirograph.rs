use valora::prelude::rand::rngs::ThreadRng;
use valora::prelude::*;
mod tools;
mod types;
use std::iter;

// cargo run -- -w 1200 -r 60
fn main() -> Result<()> {
    run_fn(Options::from_args(), |_, world, _| {
        let theta = 0.3;
        let max_width = world.width / 2.;

        let min_out_circle = 150;
        let max_out_circle = max_width as u32 - 50;
        let min_inn_circle = 60;
        let min_line_from_center = 20;
        let mut rand_gen: ThreadRng = rand::thread_rng();
        let calculate_rand = |rng: &mut ThreadRng, min, max| {
            tools::math::generate_random_between(rng, min, max) as f32
        };
        let calculate_max_steps = |out: f32, inner: f32| tools::math::calculate_lcm(out, inner);

        let mut out_circ_radius: f32 =
            calculate_rand(&mut rand_gen, min_out_circle, max_out_circle);
        let mut inn_circ_radius: f32 =
            calculate_rand(&mut rand_gen, min_inn_circle, out_circ_radius as u32);
        let mut line_from_center: f32 = calculate_rand(
            &mut rand_gen,
            min_line_from_center,
            (max_width - inn_circ_radius) as u32,
        ) as f32;

        let mut angle: f32 = 0.0;
        let mut max_angle = calculate_max_steps(out_circ_radius, inn_circ_radius);

        let center_point = world.center();
        let empty_point = P2::new(0.0, 0.0);
        let mut last = empty_point;
        let mut color_point = LinSrgb::new(0., 0.3, 0.4);

        Ok(move |ctx: Context, canvas: &mut Canvas| {
            if angle > max_angle {
                last = empty_point;
                angle = 0.;
                out_circ_radius = calculate_rand(&mut rand_gen, min_out_circle, max_out_circle);
                inn_circ_radius =
                    calculate_rand(&mut rand_gen, min_inn_circle, out_circ_radius as u32);
                line_from_center = calculate_rand(
                    &mut rand_gen,
                    min_line_from_center,
                    (max_width - inn_circ_radius) as u32,
                ) as f32;
                max_angle = calculate_max_steps(out_circ_radius, inn_circ_radius);
                let mut rand_colors =
                    iter::repeat_with(|| calculate_rand(&mut rand_gen, 0, 100) * 0.01).take(3);

                color_point = LinSrgb::new(
                    rand_colors.next().unwrap(),
                    rand_colors.next().unwrap(),
                    rand_colors.next().unwrap(),
                );
            } else {
                angle += theta;

                let x = (out_circ_radius - inn_circ_radius) * angle.cos()
                    + line_from_center
                        * (((out_circ_radius - inn_circ_radius) / inn_circ_radius) * angle).cos();
                let y = (out_circ_radius - inn_circ_radius) * angle.sin()
                    - line_from_center
                        * (((out_circ_radius - inn_circ_radius) / inn_circ_radius) * angle).sin();

                let point = P2::new(x + center_point.x, y + center_point.y);
                if last.x != 0.0 && last.y != 0.0 {
                    canvas.set_color(color_point);
                    canvas.move_to(last);
                    canvas.line_to(point);
                    canvas.set_stroke_width(1.);
                    canvas.stroke();
                }
                last = point;
            }
        })
    })
}
