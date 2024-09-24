#[cfg(test)]
mod create_folders {

    use mks::make_dir;
    use std::path::Path;

    #[test]
    fn make_dir_fn() {
        const PATH_TO_CREATE: &str = "content";
        _ = make_dir(PATH_TO_CREATE);
        assert!(Path::new(PATH_TO_CREATE).exists());
    }
}
