macro_rules! inner {
    (my_token) => {
        struct Foo;
    }
}

macro_rules! outer {
    ($val:ty) => {
        #[reexpand::reexpand]
        inner!($val);
    }
}


outer!(my_token);

fn main() {
}
