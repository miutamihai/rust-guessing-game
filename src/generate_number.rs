use rand::Rng;

pub fn run() -> u32 {
    rand::thread_rng().gen_range(1..101)
}
