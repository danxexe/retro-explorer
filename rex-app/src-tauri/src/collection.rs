pub mod scanned_file;
pub mod scanner;
pub mod persistence_manager;
pub mod rcheevos;

trait NormalizePath {
    fn normalize_path(&self) -> String;
}

impl<'a> NormalizePath for &'a str {
    fn normalize_path(&self) -> String {
        self.replace('\\', "/")
    }
}

impl NormalizePath for String {
    fn normalize_path(&self) -> String {
        self.as_str().normalize_path()
    }
}
