
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


