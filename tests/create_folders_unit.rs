#[cfg(test)]
mod create_folders {

    use mks::make_dir;
    use std::path::Path;

    const PATH_TO_CREATE: &str = "content";
    const NON_EXISTANT_PATH: &str = "sample";

    #[test]
    fn make_dir_fn() {
        _ = make_dir(PATH_TO_CREATE);
        assert!(Path::new(PATH_TO_CREATE).exists());
    }

    #[test]
    fn make_dir_fn_with_error() {
        _ = make_dir(PATH_TO_CREATE);
        assert!(!Path::new(NON_EXISTANT_PATH).exists());
    }
}
