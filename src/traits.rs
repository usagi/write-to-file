use std::path::Path;

/// trait version of `write_to_file`.
/// See also: `write_to_file::write_to_file`
pub trait WriteToFile
{
 fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error>;
}

/// impl Vec<u8> version of trait of `write_to_file`.
/// See also: `write_to_file::write_to_file`
impl WriteToFile for Vec<u8>
{
 fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error>
 {
  crate::write_to_file(path, self)
 }
}

/// impl &[u8] version of trait of `write_to_file`.
/// See also: `write_to_file::write_to_file`
impl WriteToFile for &[u8]
{
 fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error>
 {
  crate::write_to_file(path, self)
 }
}

/// impl String version of trait of `write_to_file`.
/// See also: `write_to_file::write_to_file`
impl WriteToFile for String
{
 fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error>
 {
  crate::write_to_file(path, self)
 }
}

/// impl &str version of trait of `write_to_file`.
/// See also: `write_to_file::write_to_file`
impl WriteToFile for &str
{
 fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error>
 {
  crate::write_to_file(path, self)
 }
}
