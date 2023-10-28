fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn listing_1_3() {
        let x1 = 42;
        let y1 = Box::new(84);

        {
            let z = (x1, y1);
        }

        let x2 = x1;
        assert_eq!(x1, 42);

        // let y2 = y1;
    }

    #[test]
    fn listing_1_7() {
        fn replace_with_84(s: &mut Box<i32>) {
            // let was = *s;

            let was = std::mem::take(s);
            assert_eq!(*was, 42);
            assert_eq!(*s, Box::new(0)); // changed to default value
        }

        let mut s = Box::new(42);
        replace_with_84(&mut s);
    }
}
