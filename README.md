# halo
elm inspired state manager in rust

# run example 
```bash
git clone github.com/AlexanderChen1989/halo
cd halo 
cargo run --example counter
```

# Understand Example
* define Model
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
* define Msg and update to handle Msg
```rust
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

* 
