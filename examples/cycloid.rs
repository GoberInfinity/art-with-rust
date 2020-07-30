// Original code written by: Jesus Ivan Rivera
// https://youtu.be/qhbuKbxJsk8
use art::types;
use valora::prelude::*;

// cargo run --example cycloid -- -r 60
fn main() -> Result<()> {
    run_fn(Options::from_args(), |_, world, _| {
        let root_unit = |element: usize, theta: f32| {
            let two_pi = 2. * PI;
            let calc = two_pi / 200 as f32 * element as f32 + theta;
            let x = calc.cos();
            let y = calc.sin();
            (x, y)
        };

        let root_units = |size: usize, theta: f32| {
            let mut all_points: Vec<P2> = Vec::with_capacity(size);
            for current in 0..size {
                let point = root_unit(current, theta);
                all_points.push(P2::new(point.0, point.1));
            }
            all_points
        };

        let colors = vec![
            LinSrgb::new(0., 0.7, 0.8),
            LinSrgb::new(0.90000004, 0., 0.4),
            LinSrgb::new(0., 0.7, 0.8),
        ];

        let mut theta = 0.;
        let radius = 160.;
        let k = 6;
        let number_points = 200;
        let principal_points = root_units(number_points, theta);

        let background =
            types::figure::Background::new((0.01, 0.01, 0.01), world.width, world.height);
        let center_point = world.center();

        Ok(move |ctx: Context, canvas: &mut Canvas| {
            //Color of the background
            canvas.set_color(background.color);
            canvas.paint(Filled(ctx.world));
            canvas.set_stroke_width(0.1);

            for current in 0..=2 {
                for (count, point) in principal_points.iter().enumerate() {
                    let x = point.x;
                    let y = point.y;
                    let r1 = radius
                        - radius / 2.
                            * (theta + PI / 50. * (count % 40) as f32 + 0.02 * current as f32)
                                .sin();
                    let ns = root_unit(count * k % number_points, theta);
                    canvas.set_color(colors[current]);
                    canvas.move_to(P2::new(r1 * x + center_point.x, r1 * y + center_point.y));
                    canvas.line_to(P2::new(
                        r1 * ns.0 + center_point.x,
                        r1 * ns.1 + center_point.y,
                    ));
                    canvas.close_path();
                    canvas.stroke();
                }
            }

            theta += 0.01;
        })
    })
}
