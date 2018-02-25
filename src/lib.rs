#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Val(i64);

struct Test {
    val: Val
}

impl Test {
    fn val(&self) -> Val {
        self.val
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test = Test { val: Val(123) };

        assert_eq!(test.val(), Val(123));
    }
}
