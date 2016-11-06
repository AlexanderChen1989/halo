extern crate halo;

mod stores;
mod counter;

use stores::Stores;
use counter::{Model, Msg};

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
