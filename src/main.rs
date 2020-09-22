use x;

fn main() {
    let mut patina = x::Instance::new();
    if let Err(e) = patina.setup() {
        panic!("Failed to set up X Screen! {:?}", e);
    }
    patina.run();
}
