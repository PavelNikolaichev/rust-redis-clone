#[cfg(test)]
mod resp_parser {
    use codecrafters_redis::resp::protocol::{deserialize, RespType};
    use std::io;

    #[test]
    fn parse_simple_string() -> io::Result<()> {
        let input = b"+OK\r\n";
        let result = deserialize(input);
        assert!(result.is_ok());

        let deserialized_input = result?;

        assert_eq!(
            deserialized_input.0,
            RespType::SimpleString("OK".to_string())
        );
        assert_eq!(deserialized_input.1, input.len());

        Ok(())
    }

    #[test]
    fn parse_error() -> io::Result<()> {
        let input = b"-Error message\r\n";
        let result = deserialize(input);

        assert!(result.is_ok());

        let deserialized_input = result?;
        assert_eq!(
            deserialized_input.0,
            RespType::Error("Error message".to_string())
        );
        assert_eq!(deserialized_input.1, input.len());

        Ok(())
    }

    #[test]
    fn parse_integer() -> io::Result<()> {
        let input = b":42\r\n";
        let result = deserialize(input);

        assert!(result.is_ok());

        let deserialized_input = result?;
        assert_eq!(deserialized_input.0, RespType::Integer(42));
        assert_eq!(deserialized_input.1, input.len());

        Ok(())
    }

    #[test]
    fn parse_negative_integer() -> io::Result<()> {
        let input = b":-42\r\n";
        let result = deserialize(input);

        assert!(result.is_ok());

        let deserialized_input = result?;
        assert_eq!(deserialized_input.0, RespType::Integer(-42));
        assert_eq!(deserialized_input.1, input.len());

        Ok(())
    }

    #[test]
    fn parse_bulk_string() -> io::Result<()> {
        let input = b"$5\r\nhello\r\n";
        let result = deserialize(input);

        assert!(result.is_ok());

        let deserialized_input = result?;
        assert_eq!(
            deserialized_input.0,
            RespType::BulkString(Some("hello".to_string()))
        );
        assert_eq!(deserialized_input.1, input.len());

        Ok(())
    }

    #[test]
    fn parse_incomplete_bulk_string() -> io::Result<()> {
        let input = b"$5\r\nhel";
        let result = deserialize(input);
        
        assert!(result.is_err());
        
        let error = result.err().unwrap();
        assert_eq!(error.kind(), io::ErrorKind::UnexpectedEof);
        assert_eq!(error.to_string(), "Incomplete bulk string");
        
        Ok(())
    }

    #[test]
    fn parse_empty_bulk_string() -> io::Result<()> {
        let input = b"$-1\r\n";
        let result = deserialize(input);
        
        assert!(result.is_ok());
        
        let deserialized_input = result?;
        assert_eq!(deserialized_input.0, RespType::BulkString(None));
        assert_eq!(deserialized_input.1, input.len());
        
        Ok(())
    }

    #[test]
    fn parse_array() -> io::Result<()> {
        let input = b"*4\r\n+one\r\n:-42\r\n+two\r\n$5\r\nthree\r\n";
        let result = deserialize(input);
        
        assert!(result.is_ok());
        
        let deserialized_input = result?;
        assert_eq!(
            deserialized_input.0,
            RespType::Array(vec![
                RespType::SimpleString("one".to_string()),
                RespType::Integer(-42),
                RespType::SimpleString("two".to_string()),
                RespType::BulkString(Some("three".to_string())),
            ])
        );
        
        assert_eq!(deserialized_input.1, input.len());
        
        Ok(())
    }
}

#[cfg(test)]
mod resp_serializer {
    use codecrafters_redis::resp::protocol::{serialize, RespType};

    #[test]
    fn serialize_simple_string() {
        let input = RespType::SimpleString("OK".to_string());
        let serialized_input = serialize(&input);

        assert_eq!(serialized_input, b"+OK\r\n");
    }

    #[test]
    fn serialize_error() {
        let input = RespType::Error("Error message".to_string());
        let serialized_input = serialize(&input);

        assert_eq!(serialized_input, b"-Error message\r\n");
    }

    #[test]
    fn serialize_integer() {
        let input = RespType::Integer(42);
        let serialized_input = serialize(&input);

        assert_eq!(serialized_input, b":42\r\n");
    }

    #[test]
    fn serialize_negative_integer() {
        let input = RespType::Integer(-42);
        let serialized_input = serialize(&input);

        assert_eq!(serialized_input, b":-42\r\n");
    }

    #[test]
    fn serialize_bulk_string() {
        let input = RespType::BulkString(Some("hello".to_string()));
        let serialized_input = serialize(&input);

        assert_eq!(serialized_input, b"$5\r\nhello\r\n");
    }

    #[test]
    fn serialize_empty_bulk_string() {
        let input = RespType::BulkString(None);
        let serialized_input = serialize(&input);

        assert_eq!(serialized_input, b"$-1\r\n");
    }

    #[test]
    fn serialize_incomplete_bulk_string() {
        let input = RespType::BulkString(Some("hel".to_string()));
        let serialized_input = serialize(&input);

        assert_eq!(serialized_input, b"$3\r\nhel\r\n");
    }

    #[test]
    fn serialize_array() {
        let input = RespType::Array(vec![
            RespType::SimpleString("one".to_string()),
            RespType::SimpleString("two".to_string()),
            RespType::Integer(42),
            RespType::BulkString(Some("three".to_string())),
        ]);
        let serialized_input = serialize(&input);

        assert_eq!(
            serialized_input,
            b"*4\r\n+one\r\n+two\r\n:42\r\n$5\r\nthree\r\n"
        );
    }
}
