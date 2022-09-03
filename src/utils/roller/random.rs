use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

pub fn roll(amount: u8, size: u8) -> Vec<u8>
{
    let distribution = Uniform::new_inclusive(1, size);
    let mut rng = thread_rng();

    let rolls: Vec<u8> = (0..amount).map(|_| distribution.sample(&mut rng)).collect();

    rolls
}
