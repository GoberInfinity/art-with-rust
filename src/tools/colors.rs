use valora::prelude::Context;

pub fn oscillate_using(ctx: &Context, base: f32, limit: f32) -> f32 {
    ctx.time.as_secs_f32().cos().abs() * limit + base
}
