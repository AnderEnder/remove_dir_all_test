#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn it_works() {
        let target = tempfile::Builder::new()
            .prefix("s3_download")
            .tempdir()
            .unwrap()
            .path()
            .to_path_buf();

        let data = "Some data!";
        let file = target.join("test.txt");

        let dir_path = file.parent().unwrap();
        fs::create_dir_all(&dir_path).unwrap();

        fs::write(&file, data).expect("Unable to write file");

        remove_dir_all::remove_dir_all(&target).unwrap();
    }
}
