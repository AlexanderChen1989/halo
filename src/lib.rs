
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