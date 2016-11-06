#![allow(warnings)]

struct Store<M, S> {
    keyGen: i64,
    model: M,
    update: fn(S, &M) -> M,
    subscriptions: Vec<Subscribe<M>>,
}

// type Subscribe<M> = fn(&M);

struct Subscribe<M> {
    key: i64,
    subscribe: fn(&M),
}

impl<M: Eq, S> Store<M, S> {
    fn new(model: M, update: fn(S, &M) -> M) -> Store<M, S> {
        Store {
            keyGen: 0,
            model: model,
            update: update,
            subscriptions: vec![],
        }
    }

    fn dispatch(&mut self, msg: S) {
        let new = (self.update)(msg, &self.model);
        if new != self.model {
            self.model = new;
            self.run();
        }
    }

    fn run(&self) {
        for sub in &self.subscriptions {
            (sub.subscribe)(&self.model)
        }
    }

    fn subscribe(&mut self, sub: fn(&M)) -> i64 {
        self.keyGen += 1;
        self.subscriptions.push(Subscribe {
            key: self.keyGen,
            subscribe: sub,
        });
        return self.keyGen;
    }

    fn unsubscribe(&mut self, sub: i64) {
        let index = self.subscriptions.iter().position(|x| x.key == sub);
        if let Some(i) = index {
            self.subscriptions.remove(i);
        }
    }
}

#[derive(Debug)]
enum Msg {
    Incr(i64),
    Decr(i64),
}

type Model = i64;

fn update(msg: Msg, model: &Model) -> Model {
    match msg {
        Msg::Incr(v) => (model + v),
        Msg::Decr(v) => (model - v),
    }
}

fn subscribe(model: &Model) {
    println!("Hello, {}!", model);
}

fn main() {
    let mut store = Store::new(10, update);
    let sub1 = store.subscribe(subscribe);
    store.run();
    store.dispatch(Msg::Incr(10));
    store.dispatch(Msg::Incr(10));
    store.dispatch(Msg::Incr(10));
}