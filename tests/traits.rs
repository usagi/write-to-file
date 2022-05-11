#[test]
fn traits()
{
 use std::fs::read_to_string;
 use write_to_file::WriteToFile;

 // Vec<u8>
 let buf: Vec<u8> = vec![1u8, 2, 3, 4];
 let expected = buf.clone();
 let path = "target/test/file.bin";
 buf.write_to_file(path).unwrap();
 let actual = read_to_string(path).unwrap().as_bytes().to_vec();
 assert_eq!(expected, actual);

 // &[u8]
 let buf: &[u8] = buf.as_slice();
 let expected = buf.clone();
 let path = "target/test/file.bin";
 buf.write_to_file(path).unwrap();
 let actual = read_to_string(path).unwrap().as_bytes().to_vec();
 assert_eq!(expected, actual);

 // String
 let buf: String = "Nyanko is one of the greatest life.".to_string();
 let expected = buf.clone();
 let path = "target/test/file.txt";
 buf.write_to_file(path).unwrap(); // or &buf simply
 let actual = read_to_string(path).unwrap();
 assert_eq!(expected, actual);

 // &str
 let buf: &str = buf.as_str();
 let expected = buf.clone();
 let path = "target/test/file.txt";
 buf.write_to_file(path).unwrap(); // or &buf simply
 let actual = read_to_string(path).unwrap();
 assert_eq!(expected, actual);
}
