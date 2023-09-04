mod systems;

pub mod prelude {
    pub use std::io::stdin;
    pub use tokio::time::{sleep, Duration};
    pub use rand::{Rng, rngs::ThreadRng};

    pub use crate::systems::*;
}
