mod counter;

use counter::*;

pub struct Stores {
    a: Store<Model, Msg>,
    b: Store<Model, Msg>,
}

impl Stores {
    pub fn new() -> Stores {
        Stores {
            a: Store::new(Model::new(10), update),
            b: Store::new(Model::new(20), update),
        }
    }
}