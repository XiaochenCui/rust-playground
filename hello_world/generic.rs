fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            larget = item;
        }
    }

    largest
}

fn main() {
    let nu
}