#[cfg(test)]
use mocktopus::macros::*;

fn world() -> &'static str {
    "world"
}

#[cfg_attr(test, mockable)]
pub fn hello_world() -> String {
    format!("Hello {}!", world())
}


#[cfg(test)]
mod tests {
    use super::hello_world;
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
