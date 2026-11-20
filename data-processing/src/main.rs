trait DataPipeline<T> {
    fn insert(&mut self, item: T);

    fn insert_many(&mut self, items: impl Iterator<Item = T>); //the impl Iterator thing means that items must be of some type that implements the iterator trait

    fn num_valid(&self) -> u32; //number of valid items in the pipeline/number of items that have passed the filter method

    // return any items from pipeline that fail the predicates test
    fn filter(&mut self, predicate: fn(&T) -> bool); // the fn means it accepts a function as the predicate perameter
}

struct Validator<T> {
    items: Vec<T>,
}

impl<T> Validator<T> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T> DataPipeline<T> for Validator<T> {
    fn insert(&mut self, item: T) {
        self.items.push(item);
    }

    fn insert_many(&mut self, items: impl Iterator<Item = T>) {
        self.items.extend(items);
    }

    fn num_valid(&self) -> u32 {
        self.items.len() as u32
    }

    fn filter(&mut self, predicate: fn(&T) -> bool) {
        self.items.retain(predicate);
    }
}

fn main() {
    let mut vals = Validator::<i32>::new();
    vals.insert(1);
    vals.insert(10);
    vals.insert(100);
    vals.filter(|&x| x < 50);

    assert_eq!(vals.num_valid(), 2);
}
