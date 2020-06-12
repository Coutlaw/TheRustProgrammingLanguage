pub struct ThreadPool;

impl ThreadPool {
    pub fn new(_size: usize) -> ThreadPool {
        ThreadPool
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
