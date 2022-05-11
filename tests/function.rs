#[test]
fn function()
{
 use std::fs::read_to_string;
 use write_to_file::write_to_file;

 // write binary
 let buf = vec![1u8, 2, 3, 4];
 let expected = buf.clone();
 let path = "target/test/file.bin";
 write_to_file(path, buf).unwrap();
 let actual = read_to_string(path).unwrap().as_bytes().to_vec();
 assert_eq!(expected, actual);

 // write text
 let buf = "Nyanko is one of the greatest life.".to_string();
 let expected = buf.clone();
 let path = "target/test/file.txt";
 write_to_file(path, buf).unwrap(); // or &buf simply
 let actual = read_to_string(path).unwrap();
 assert_eq!(expected, actual);
}
