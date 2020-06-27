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


#[cfg(test)]
mod tests {
    use super::hello_world::hello_world;
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
}


