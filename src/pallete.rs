use valora::prelude::*;
mod types;

pub struct NewsArticle {
    pub pallet: Vec<(f32, f32, f32)>,
}

impl Artist for NewsArticle {
    fn setup(gpu: Gpu, world: World, rng: &mut StdRng) -> Result<Self> {
        let mut results: Vec<(f32, f32, f32)> = Vec::new();
        let rate: f32 = 1. / 10.;
        for r in 0..10 {
            for g in 0..10 {
                for b in 0..10 {
                    results.push((r as f32 * rate, g as f32 * rate, b as f32 * rate));
                }
            }
        }
        let user1 = NewsArticle { pallet: results };
        Ok(user1)
    }
    fn paint(&mut self, ctx: Context, canvas: &mut Canvas) {
        let mut col = 0;
        for index in 0..=self.pallet.len() - 1 {
            let current_color = self.pallet[index];
            let row = index % 100;

            if index != 0 && row == 0 {
                col += 1;
            }
            let color = LinSrgb::new(current_color.0, current_color.1, current_color.2);
            canvas.set_color(color);
            canvas.paint(Filled(forms::Ngon::square(
                P2::new((row as f32) * 15. + 50., (col as f32) * 20. + 50.),
                6.,
            )));
        }
    }
}

// cargo run -- -w 1200 -r 60
fn main() -> Result<()> {
    run::<NewsArticle>(Options::from_args())
}
