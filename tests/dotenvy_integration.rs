#[cfg(test)]
mod environment {

    use std::env;

    #[test]
    fn valid_environment_variable() {
        env::set_var("SAMPLE_ENV", "test");
        assert_eq!(env::var("SAMPLE_ENV").unwrap(), "test");
    }

    #[test]
    fn missing_environment_variable() {
        env::set_var("SAMPLE_ENV", "");
        assert_ne!(env::var("SAMPLE_ENV").unwrap(), "test");
    }
}
