use std::io::{ self, Cursor, BufRead, BufReader };
use quickcheck_macros::quickcheck;
use fast_lines::ReadLine;


fn read_line<A: AsRef<[u8]>>(input: &A) -> io::Result<Vec<String>> {
    let reader = Cursor::new(input.as_ref());
    let mut readline = ReadLine::new(reader);
    let mut output = Vec::new();

    while let Some(line) = readline.read_line()? {
        output.push(line.to_string());
    }

    Ok(output)
}

fn std_read_line<A: AsRef<[u8]>>(input: &A) -> io::Result<Vec<String>> {
    BufReader::new(input.as_ref())
        .lines()
        .collect()
}

#[test]
fn test_line() -> io::Result<()> {
    macro_rules! assert_line {
        ( $input:expr ) => {
            assert_eq!(read_line($input)?, std_read_line($input)?);
        };
        ( $( $input:expr $(;)? )* ) => {
            $(
                assert_line!(&$input);
            )*
        }
    }

    assert_line!{
        b"";
        b"a";
        b"a\nb";
        b"\n";
        b"\na";
        b"a\n";
        b"a\nb\n";
        b"\n\n";
        b"a\n\nb";
        vec![b'\n'; 1025];
        vec![b'a'; 1026];
        b"\r\n";
        b"a\r";
        b"a\r\n";
        b"a\n\r";
        b"a\r\nb";
        b"b\r\na";
        b"\r\r\n\r";
        {
            let mut input = Vec::new();
            input.push(b'a');
            input.extend_from_slice(&[b'\n'; 1024][..]);
            input.push(b'a');
            input
        };
        {
            let mut input = Vec::new();
            input.push(b'\n');
            input.push(b'a');
            input.extend_from_slice(&[b'a'; 1024][..]);
            input.push(b'\n');
            input
        };
        {
            let mut input = Vec::new();
            input.push(b'a');
            input.push(b'\n');
            input.extend_from_slice(&[b'a'; 512][..]);
            input.push(b'\n');
            input.extend_from_slice(&[b'a'; 32][..]);
            input.push(b'\n');
            input.extend_from_slice(&[b'a'; 128][..]);
            input.push(b'\n');
            input.extend_from_slice(&[b'a'; 768][..]);
            input.push(b'\n');
            input.push(b'a');
            input
        };
    };


    Ok(())
}

#[quickcheck]
fn test_quickcheck(input: Vec<u8>) -> bool {
    read_line(&input).unwrap() == std_read_line(&input).unwrap()
}
