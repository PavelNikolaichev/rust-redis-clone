/// Integration tests for redis commands
#[cfg(test)]
mod test_ping {
    use codecrafters_redis::resp::commands::{Command, DefaultServerState, Ping};
    use codecrafters_redis::resp::protocol::RespType;

    #[test]
    fn test_ping() {
        let mut state = DefaultServerState::default();

        let cmd = Ping;
        let result = cmd.execute(&[], &mut state).unwrap();

        assert_eq!(result, RespType::SimpleString("PONG".to_string()));
    }
}

#[cfg(test)]
mod test_echo {
    use codecrafters_redis::resp::commands::{Command, DefaultServerState, Echo};
    use codecrafters_redis::resp::protocol::RespType;

    #[test]
    fn test_echo_one_arg() {
        let mut state = DefaultServerState::default();

        let cmd = Echo;
        let args = vec![RespType::BulkString(Option::from(
            "Hello, World!".to_string(),
        ))];

        let result = cmd.execute(&args, &mut state).unwrap();

        assert_eq!(
            result,
            RespType::BulkString(Option::from("Hello, World!".to_string()))
        );
    }

    #[test]
    fn test_echo_no_args() {
        let mut state = DefaultServerState::default();

        let cmd = Echo;
        let args: Vec<RespType> = vec![];

        let result = cmd.execute(&args, &mut state);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "ECHO requires at least one argument".to_string()
        );
    }

    #[test]
    fn test_echo_multiple_args() {
        let mut state = DefaultServerState::default();

        let cmd = Echo;
        let args = vec![
            RespType::BulkString(Option::from("Hello".to_string())),
            RespType::BulkString(Option::from("World".to_string())),
        ];

        let result = cmd.execute(&args, &mut state).unwrap();

        assert_eq!(
            result,
            RespType::BulkString(Option::from("Hello".to_string()))
        );
    }

    #[test]
    fn test_echo_empty_string() {
        let mut state = DefaultServerState::default();

        let cmd = Echo;
        let args = vec![RespType::BulkString(Option::from("".to_string()))];

        let result = cmd.execute(&args, &mut state).unwrap();

        assert_eq!(result, RespType::BulkString(Option::from("".to_string())));
    }

    #[test]
    fn test_echo_non_string_arg() {
        let mut state = DefaultServerState::default();

        let cmd = Echo;
        let args = vec![RespType::Integer(42)];

        let result = cmd.execute(&args, &mut state);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "ECHO argument must be a string".to_string()
        );
    }
}
