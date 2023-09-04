mod systems;

pub mod prelude {
    pub use rand::{rngs::ThreadRng, Rng};
    pub use std::io::stdin;
    pub use tokio::time::{sleep, Duration};

    pub use crate::systems::*;
}
