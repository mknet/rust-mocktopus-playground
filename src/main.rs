extern crate mocktopus;

#[cfg(test)]
use mocktopus::macros::*;

fn main() {
    println!("{}", hello_world::hello_world());
}

#[mockable]
mod hello_world {
    pub fn world() -> &'static str {
        "world"
    }

    pub fn hello_world() -> String {
        format!("Hello {}!", world())
    }
}

mod under_test {
    use super::hello_world::hello_world;

    pub fn do_something() -> String {
        hello_world()
    }
}


#[cfg(test)]
mod tests {
    use super::hello_world::hello_world;
    use super::under_test::do_something;
    use mocktopus::mocking::*;

    #[test]
    fn test_helloworld() {
        assert_eq!(hello_world(), "Hello world!");
    }

    #[test]
    fn mock_helloworld() {

        hello_world.mock_safe(|| MockResult::Return(String::from("Gude")));
        assert_eq!(hello_world(), "Gude");
    }

    #[test]
    fn under_test() {

        hello_world.mock_safe(|| MockResult::Return(String::from("Hello Mock")));


        assert_eq!(do_something(), "Hello Mock");
    }
}


