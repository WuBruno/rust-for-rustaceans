#![allow(unused)]

mod rust_lang;

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
            assert_eq!(*s, Box::new(0)); // changed to default value of 0

            *s = was;
            assert_eq!(**s, 42);

            let mut r = Box::new(84);
            std::mem::swap(s, &mut r);
            assert_eq!(*r, 42);
            assert_eq!(**s, 84);
        }

        let mut s = Box::new(42);
        replace_with_84(&mut s);
    }

    #[test]
    fn listing_1_8() {
        fn rand() -> f32 {
            0.1
        }

        let mut x = Box::new(42);
        let r = &x;
        if rand() > 0.5 {
            *x = 84;
            // dbg!(r);
        } else {
            dbg!(r);
        }

        // dbg!(r);
        dbg!(x);
    }

    #[test]
    fn listing_1_9() {
        let mut x = Box::new(42);
        let mut z = &x;
        for i in 0..100 {
            dbg!(z);
            x = Box::new(i);
            // dbg!(z); illegal – lifetime dead here
            z = &x; // resumed here. Without this it would not work
        }

        dbg!(z);
    }

    #[test]
    fn listing_1_10() {
        struct StrSplit<'s, 'p> {
            delimeter: &'p str,
            document: &'s str,
        }
        impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
            type Item = &'s str;

            fn next(&mut self) -> Option<Self::Item> {
                todo!()
            }
        }
        fn str_before(s: &str, c: char) -> Option<&str> {
            StrSplit {
                document: s,
                delimeter: &c.to_string(),
            }
            .next()
        }
    }

    #[test]
    fn listing_1_11() {
        struct MutStr<'a, 'b> {
            s: &'a mut &'b str,
        }

        let mut s2 = "hello";
        *(MutStr { s: &mut s2 }.s) = "world";
        dbg!(s2);
    }
}
