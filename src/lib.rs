use utilities::adsr;

pub mod functions;
mod audio_processor;
mod audio_source;
pub mod sources;
pub mod processors;
pub mod utilities;
pub mod consts;

#[cfg(test)]
mod tests {    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
