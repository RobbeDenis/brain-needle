
// using
use crate::bncore::DEFAULT_OUT_DIR;
use crate::bncore::DEFAULT_OUT_FILE;
use crate::bndest::BNDest;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct FileDest
{
    output: String,
    path: Option<PathBuf>
}

impl FileDest
{
    pub const fn new(path: Option<PathBuf>) -> FileDest
    {
        return FileDest{ output: String::new(), path };
    }
}

impl BNDest for FileDest
{
    fn push(&mut self, data: &[u8])
    {
        if let Ok(s) = std::str::from_utf8(data) {
        self.output.push_str(s);
    }
    }

    fn finalize(&self)
    {
        let mut file = match &self.path {
            None => File::create(format!("{}\\{}", DEFAULT_OUT_DIR, DEFAULT_OUT_FILE)).unwrap(),
            Some(p) => File::create(p).unwrap()
        };

        file.write_all(self.output.as_bytes()).unwrap();
    }
}