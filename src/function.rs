use std::{
 fs,
 io::Write,
 path::Path
};

/// Write `buf` to `path`, with create required directories if not exists.
/// ```rust
/// // write binary
/// let buf = vec![1u8, 2, 3, 4]; // Both of Vec<u8> and &[u8] are supported.
/// let path = "target/test/path/to/file.bin";
/// write_to_file::write_to_file(path, buf);
/// // write text
/// let buf = "Nyanko is one of the greatest life."; // Both of String and str are supported.
/// let path = "target/test/path/to/file.txt";
/// write_to_file::write_to_file(path, buf);
/// ```
pub fn write_to_file<P, T>(path: P, buf: T) -> std::io::Result<()>
where
 P: AsRef<Path>,
 T: AsRef<[u8]>
{
 let path = path.as_ref();

 if let Some(dir) = path.parent()
 {
  fs::create_dir_all(dir)?;
 }

 fs::OpenOptions::new()
  .write(true)
  .create(true)
  .truncate(true)
  .open(path)?
  .write_all(buf.as_ref())
}
