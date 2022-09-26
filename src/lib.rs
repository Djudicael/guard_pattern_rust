use std::cell::RefCell; // enable the mutability during the run time

struct Pool<T> {
    items: RefCell<Vec<T>>,
}

impl<T: PoolItem> Pool<T> {
    fn new() -> Self {
        Self {
            items: RefCell::new(Vec::new()),
        }
    }
    fn get(&self) -> PoolGuard<T> {
        //borrow_mut keep the borrowing in check
        let item = match self.items.borrow_mut().pop() {
            Some(item) => item,
            None => T::new(),
        };
        PoolGuard {
            inner: Some(item),
            items: &self.items,
        }
    }
}

trait PoolItem {
    fn new() -> Self;
    fn reset(&mut self);
}

struct PoolGuard<'a, T: PoolItem> {
    inner: Option<T>,
    items: &'a RefCell<Vec<T>>,
}

impl<T: PoolItem> Drop for PoolGuard<'_, T> {
    fn drop(&mut self) {
        let mut item = self.inner.take().unwrap();

        item.reset();

        // somehow return the item to the Pool
        self.items.borrow_mut().push(item);
    }
}

impl<T: PoolItem> std::ops::Deref for PoolGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<T: PoolItem> std::ops::DerefMut for PoolGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        struct Awesome(usize);
        impl Awesome {
            fn get(&self) -> usize {
                self.0
            }

            fn inc(&mut self) {
                self.0 += 1;
            }
        }
        impl PoolItem for Awesome {
            fn new() -> Self {
                Awesome(0)
            }

            fn reset(&mut self) {
                self.0 = 0;
            }
        }
        let pool = Pool::<Awesome>::new();
        let mut item = pool.get();
        item.inc();

        assert_eq!(item.get(), 1);

        drop(item);

        let item_2 = pool.get();

        assert_eq!(item_2.get(), 0);
    }
}
