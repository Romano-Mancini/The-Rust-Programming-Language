#[derive(Debug)]
struct Rectangle {
    h: i32,
    w: i32,
}
impl Rectangle {
    pub fn can_hold(&self, another: &Rectangle) -> bool {
        another.h <= self.h && another.w <= self.w
    }

    pub fn get_height(&self) -> i32 {
        self.h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_holds_smaller() {
        let larger = Rectangle { w: 10, h: 10 };
        let smaller = Rectangle { w: 5, h: 5 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_does_not_hold_larger() {
        let larger = Rectangle { w: 10, h: 10 };
        let smaller = Rectangle { w: 5, h: 5 };

        assert!(
            !smaller.can_hold(&larger),
            "The two rectangles were {larger:#?} {smaller:#?}"
        );
    }

    #[test]
    fn get_height_returns_height() {
        let rect = Rectangle { w: 15, h: 10 };

        assert_eq!(rect.get_height(), 10);
    }

    #[test]
    fn get_height_returns_width() {
        let rect = Rectangle { w: 15, h: 10 };

        assert_ne!(rect.get_height(), 15);
    }

    #[test]
    #[should_panic] // #[should_panic(expected = "The test harness will make sure that the failure message contains the provided text")]
    fn get_outside_range() {
        let vector = vec![1, 2, 3];
        let reference = &vector[10];
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert!(false)
    }
}
