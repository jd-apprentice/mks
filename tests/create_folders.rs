#[cfg(test)]
mod create_folders {

    use std::path::Path;
    use mks::make_dir;

    #[test]
    fn make_dir_fn() {
        const PATH_TO_CREATE: &str = "content";
        let _ = make_dir(PATH_TO_CREATE);
        assert!(Path::new(PATH_TO_CREATE).exists());
    }

}