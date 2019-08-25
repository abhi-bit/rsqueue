pub struct Queue<T: Clone> {
    queue: Vec<T>
}

impl <T: Clone> Queue<T> {
    pub fn new() -> Queue<T>{
        Queue{
            queue: vec![]
        }
    }

    fn add(&mut self, val: T){
        self.queue.push(val);
    }

    fn remove(&mut self) -> Result<T, &str> {
        if !self.queue.is_empty() {
            Ok(self.queue.remove(0usize))
        } else {
            Err("queue is empty")
        }
    }

    fn peek(&self) -> Result<T, &str> {
        match self.queue.first() {
            Some(val) => Ok(val.clone()),
            None => Err("queue is empty"),
        }
    }

    fn size(&self) -> usize {
        self.queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut q: Queue<isize> = Queue::new();
        q.add(1);
        q.add(2);
        assert_eq!(q.peek(), Ok(1));
        assert_eq!(q.size(), 2);
        assert_eq!(q.remove(), Ok(1));
        assert_eq!(q.remove(), Ok(2));
        assert_eq!(q.remove(), Err("queue is empty"));
    }
}
