use rand::Rng;

pub fn generate_bytes(size: u32) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}
