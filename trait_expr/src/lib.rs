use std::any::Any;

trait A {
    fn as_any(&self) -> &dyn Any;
}

struct B;

impl A for B {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct ZooKeeper {
    zoo: Vec<Box<dyn Animal>>,
}

impl ZooKeeper {
    fn add(&self, animal: impl Animal) {}
}

trait Animal {}

struct Dog {}
struct Cat {}

impl Animal for Dog {}
impl Animal for Cat {}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let mut zoo: Vec<Box<dyn Animal>> = Vec::new();
        let d = Dog {};
        let c = Cat {};
        // zoo.push(Box::new(Dog{}));
        // zoo.push(Box::new(Cat{}));

        let zk = ZooKeeper { zoo: Vec::new() };
        zk.add(d);
    }

    fn test_as_any() {
        let a: Box<dyn A> = Box::new(B);
        // The indirection through `as_any` is because using `downcast_ref`
        // on `Box<A>` *directly* only lets us downcast back to `&A` again.
        // The method ensures we get an `Any` vtable that lets us downcast
        // back to the original, concrete type.
        let b: &B = match a.as_any().downcast_ref::<B>() {
            Some(b) => b,
            None => panic!("&a isn't a B!"),
        };
    }
}