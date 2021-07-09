use utilities::adsr;

pub mod functions;
mod audio_processor;
mod audio_source;
pub mod sources;
pub mod processors;
pub mod utilities;
pub mod consts;

fn main() {
    let adsr = utilities::adsr::ADSR::new(0.0, 0.0, 1.0, 0.0, 44100.0);
}
#[cfg(test)]
mod tests {    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
