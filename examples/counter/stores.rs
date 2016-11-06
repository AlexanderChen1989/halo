
extern crate halo;

use halo::Store;
use counter::{Model, Msg, update};

pub struct Stores {
    pub a: Store<Model, Msg>,
    pub b: Store<Model, Msg>,
}

impl Stores {
    pub fn new() -> Stores {
        Stores {
            a: Store::new(Model::new(10), update),
            b: Store::new(Model::new(20), update),
        }
    }
}
