extern crate halo;

mod counter;

use counter::*;
use halo::Store;

fn main() {
    let mut stores = Stores::new();

    stores.a.subscribe(sub);
    stores.b.subscribe(sub);

    stores.a.dispatch(Msg::Incr(10));
    stores.a.dispatch(Msg::Nothing);
    stores.b.dispatch(Msg::Decr(20));
    stores.b.dispatch(Msg::Add(100));

    fn sub(model: &Model) {
        println!("Sub {:?}", model);
    }
}


struct Stores {
    a: Store<Model, Msg>,
    b: Store<Model, Msg>,
}

impl Stores {
    fn new() -> Stores {
        Stores {
            a: Store::new(Model::new(10), update),
            b: Store::new(Model::new(20), update),
        }
    }
}