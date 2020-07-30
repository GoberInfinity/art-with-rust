use std::collections::VecDeque;
use valora::prelude::*;
mod tools;
mod types;

// cargo run --example ex1 -- -w 1200 -r 60
fn main() -> Result<()> {
    run_fn(Options::from_args(), |_, world, _| {
        let mut rng = rand::thread_rng();
        let mut angle = 0.0;
        let mut number_of_vertex = 3;
        let mut is_increasing = true;

        let radius = 2.;
        let scale_by = 1.05;
        let background = types::figure::Background::new((0., 0., 0.), world.width, world.height);
        let middle_point = world.center();
        let n_gon_figure =
            types::figure::Polygon::new(0., middle_point, radius, number_of_vertex, 5., scale_by);

        let mut figures_to_render: VecDeque<_> = vec![n_gon_figure].into_iter().collect();

        Ok(move |ctx: Context, canvas: &mut Canvas| {
            let random_number: u8 = tools::math::generate_u8_rand(&mut rng);

            //Color of the background
            canvas.set_color(background.color);
            canvas.paint(Filled(ctx.world));

            //Paints all the elements to be displayed and rotates&scales all the figures
            for index in 0..=figures_to_render.len() - 1 {
                canvas.set_color(figures_to_render[index].color);
                canvas.paint(Stroked {
                    width: 1.,
                    element: figures_to_render[index].ngon,
                });
                figures_to_render[index].scale_rotate_figure();
            }

            // Every frame it creates the new seed figure
            angle += 0.1;
            let mut incoming_figure = types::figure::Polygon::new(
                0.,
                middle_point,
                radius,
                number_of_vertex,
                angle,
                scale_by,
            );
            incoming_figure.rotate();
            incoming_figure.change_color_to(tools::colors::oscillate_using(&ctx, 0., 0.4));
            figures_to_render.push_back(incoming_figure);

            //It changes randomly the number of vertex of the figure
            match random_number % 11 {
                0 => {
                    let increase_vertex_closure =
                        |is_increasing, number_of_vertex: usize| match is_increasing {
                            true => number_of_vertex + 1,
                            _ => number_of_vertex - 1,
                        };
                    number_of_vertex = increase_vertex_closure(is_increasing, number_of_vertex);
                    if number_of_vertex < 3 || number_of_vertex > 10 {
                        is_increasing = !is_increasing;
                        number_of_vertex = increase_vertex_closure(is_increasing, number_of_vertex);
                    }
                }
                _ => (),
            }

            // Disappears the figure when it goes out of the screen
            if figures_to_render.get(0).unwrap().ngon.radius > background.hypotenuse {
                figures_to_render.pop_front();
            }
        })
    })
}
