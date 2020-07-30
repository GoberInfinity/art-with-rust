use valora::prelude::rand::rngs::ThreadRng;
use valora::prelude::rand::Rng;
use valora::prelude::PI;

//https://en.wikipedia.org/wiki/Hypotrochoid
pub fn calculate_lcm(big_radius: f32, small_radius: f32) -> f32 {
    2. * PI * lcm(big_radius, small_radius) / big_radius
}

//https://en.wikipedia.org/wiki/Least_common_multiple#Applications
fn lcm(first: f32, second: f32) -> f32 {
    first / (gcd(first, second) as f32) * second
}

//https://en.wikipedia.org/wiki/Euclidean_algorithm#Worked_example
fn gcd(first: f32, second: f32) -> u32 {
    let mut first = first as u32;
    let mut second = second as u32;
    while second != 0 {
        let multiple_of_number_of_times = second;
        second = first % second;
        first = multiple_of_number_of_times;
    }
    first
}

pub fn generate_random_between(rng: &mut ThreadRng, min: u32, max: u32) -> u32 {
    rng.gen_range(min, max)
}

pub fn generate_u8_rand(rng: &mut ThreadRng) -> u8 {
    generate_random_between(rng, 0, 255) as u8
}
