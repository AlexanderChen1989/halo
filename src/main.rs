// #![allow(warnings)]


pub mod counter {
    #[derive(Debug)]
    pub enum Msg {
        Incr(i64),
        Decr(i64),
        Add(i64),
        Nothing,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Model {
        num: i64,
        others: Vec<i64>,
    }


    impl Model {
        pub fn new(num: i64) -> Model {
            Model {
                num: num,
                others: vec![1, 2, 3, 5, 6],
            }
        }
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

}

struct Stores {
    a: Store<counter::Model, counter::Msg>,
    b: Store<counter::Model, counter::Msg>,
}

impl Stores {
    fn new() -> Stores {
        Stores {
            a: Store::new(counter::Model::new(10), counter::update),
            b: Store::new(counter::Model::new(20), counter::update),
        }
    }
}

fn main() {
    let mut stores = Stores::new();

    stores.a.subscribe(sub);
    stores.b.subscribe(sub);

    stores.a.dispatch(counter::Msg::Incr(10));
    stores.b.dispatch(counter::Msg::Add(100));
    stores.a.dispatch(counter::Msg::Nothing);

    fn sub(model: &counter::Model) {
        println!("Sub {:?}", model);
    }
}

pub struct Store<M, S> {
    key: i64,
    model: M,
    update: fn(S, M) -> M,
    subscriptions: Vec<Subscribe<M>>,
}

pub struct Subscribe<M> {
    key: i64,
    subscribe: fn(&M),
}

impl<M: Clone + PartialEq, S> Store<M, S> {
    pub fn new(model: M, update: fn(S, M) -> M) -> Store<M, S> {
        Store {
            key: 0,
            model: model,
            update: update,
            subscriptions: vec![],
        }
    }

    pub fn dispatch(&mut self, msg: S) {
        let new = (self.update)(msg, self.model.clone());
        if new != self.model {
            self.model = new;
            self.run();
        }
    }

    pub fn run(&self) {
        for sub in &self.subscriptions {
            (sub.subscribe)(&self.model)
        }
    }

    pub fn subscribe(&mut self, sub: fn(&M)) -> i64 {
        self.key += 1;
        self.subscriptions.push(Subscribe {
            key: self.key,
            subscribe: sub,
        });
        return self.key;
    }

    pub fn unsubscribe(&mut self, sub: i64) {
        let index = self.subscriptions.iter().position(|x| x.key == sub);
        if let Some(i) = index {
            self.subscriptions.remove(i);
        }
    }
}