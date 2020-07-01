extern crate mocktopus;

mod under_test;
mod hello_world;

fn main() {
    println!("{}", under_test::do_something());
}

#[cfg(test)]
mod tests {
    use crate::hello_world::hello_world;
    use crate::under_test::do_something;
    use mocktopus::mocking::*;

    #[test]
    fn under_test() {

        hello_world.mock_safe(|| MockResult::Return(String::from("Hello Mock")));


        assert_eq!(do_something(), "Hello Mock");
    }
}


