# Halo
elm inspired state manager in rust

# Run Example
```bash
git clone github.com/AlexanderChen1989/halo
cd halo
cargo run --example counter
```
---
# Understand Example
## define Model
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    num: i64,
    others: Vec<i64>,
}


impl Model {
    pub fn new(num: i64) -> Model {
        Model {
            num: num,
            others: vec![],
        }
    }
}
```
## define Msg and update to handle Msg
```rust
#[derive(Debug)]
pub enum Msg {
    Incr(i64),
    Decr(i64),
    Add(i64),
    Nothing,
}

pub fn update(msg: Msg, model: Model) -> Model {
    let mut model = model;
    match msg {
        Msg::Incr(v) => {
            model.num += v;
            model
        }
        Msg::Decr(v) => {
            model.num -= v;
            model
        }
        Msg::Add(v) => {
            model.others.push(v);
            model
        }
        Msg::Nothing => model,
    }
}
```
## define store hierachy
```rust
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
```

## use stores
```rust
extern crate halo;

mod stores;
mod counter;

use stores::Stores;
use counter::{Model, Msg};

fn main() {
    let mut stores = Stores::new();

    // subsribe function to listen store's model change
    stores.a.subscribe(sub);
    stores.b.subscribe(sub);

    // dispatch action(Msg) to store
    stores.a.dispatch(Msg::Incr(10));
    stores.a.dispatch(Msg::Nothing);
    stores.b.dispatch(Msg::Decr(20));
    stores.b.dispatch(Msg::Add(100));

    fn sub(model: &Model) {
        println!("Sub {:?}", model);
    }
}
```

```bash
cargo run --example counter
...
Sub Model { num: 20, others: [] }
Sub Model { num: 0, others: [] }
Sub Model { num: 0, others: [100] }
```
