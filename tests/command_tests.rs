/// Integration tests for redis commands
#[cfg(test)]
mod test_ping {
    use codecrafters_redis::resp::commands::{Command, Ping};
    use codecrafters_redis::resp::protocol::RespType;
    use codecrafters_redis::resp::state::default_server_state::DefaultServerState;

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
    use codecrafters_redis::resp::commands::{Command, Echo};
    use codecrafters_redis::resp::protocol::RespType;
    use codecrafters_redis::resp::state::default_server_state::DefaultServerState;

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

#[cfg(test)]
mod test_set {
    use codecrafters_redis::resp::commands::{Command, Set};
    use codecrafters_redis::resp::protocol::RespType;
    use codecrafters_redis::resp::state::default_server_state::DefaultServerState;

    #[test]
    fn test_set() {
        let mut state = DefaultServerState::default();

        let cmd = Set;
        let args = vec![
            RespType::BulkString(Option::from("key".to_string())),
            RespType::BulkString(Option::from("value".to_string())),
        ];

        let result = cmd.execute(&args, &mut state).unwrap();

        assert_eq!(result, RespType::SimpleString("OK".to_string()));
    }

    #[test]
    fn test_set_no_args() {
        let mut state = DefaultServerState::default();

        let cmd = Set;
        let args: Vec<RespType> = vec![];

        let result = cmd.execute(&args, &mut state);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "SET requires at least two arguments".to_string()
        );
    }

    #[test]
    fn test_set_one_arg() {
        let mut state = DefaultServerState::default();

        let cmd = Set;
        let args = vec![RespType::BulkString(Option::from("key".to_string()))];

        let result = cmd.execute(&args, &mut state);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "SET requires at least two arguments".to_string()
        );
    }

    #[test]
    fn set_with_ttl() {
        let mut state = DefaultServerState::default();

        let cmd = Set;
        let args = vec![
            RespType::BulkString(Option::from("key".to_string())),
            RespType::BulkString(Option::from("value".to_string())),
            RespType::Integer(60), // TTL in seconds
        ];

        let result = cmd.execute(&args, &mut state).unwrap();

        assert_eq!(result, RespType::SimpleString("OK".to_string()));
    }
}

#[cfg(test)]
mod test_get {
    use codecrafters_redis::resp::commands::{Command, Get};
    use codecrafters_redis::resp::protocol::RespType;
    use codecrafters_redis::resp::state::default_server_state::DefaultServerState;

    #[test]
    fn test_get() {
        let mut state = DefaultServerState::default();

        // Set a key-value pair first
        let set_cmd = codecrafters_redis::resp::commands::Set;
        let set_args = vec![
            RespType::BulkString(Option::from("key".to_string())),
            RespType::BulkString(Option::from("value".to_string())),
        ];
        set_cmd.execute(&set_args, &mut state).unwrap();

        // Now test the GET command
        let cmd = Get;
        let args = vec![RespType::BulkString(Option::from("key".to_string()))];

        let result = cmd.execute(&args, &mut state).unwrap();

        assert_eq!(result, RespType::BulkString(Option::from("value".to_string())));
    }

    #[test]
    fn test_get_non_existent_key() {
        let mut state = DefaultServerState::default();

        let cmd = Get;
        let args = vec![RespType::BulkString(Option::from("non_existent_key".to_string()))];

        let result = cmd.execute(&args, &mut state).unwrap();

        assert_eq!(result, RespType::BulkString(None));
    }

    #[test]
    fn test_get_no_args() {
        let mut state = DefaultServerState::default();

        let cmd = Get;
        let args: Vec<RespType> = vec![];

        let result = cmd.execute(&args, &mut state);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "GET requires at least one argument".to_string()
        );
    }

    #[test]
    fn get_expired() {
        let mut state = DefaultServerState::default();

        let set_cmd = codecrafters_redis::resp::commands::Set;
        let args = vec![
            RespType::BulkString(Option::from("temp_key".to_string())),
            RespType::BulkString(Option::from("temp_value".to_string())),
            RespType::BulkString(Option::from("PX".to_string())),
            RespType::BulkString(Option::from("1000".to_string())), // 1 second expiration
        ];
        
        set_cmd.execute(&args, &mut state).unwrap();

        // Wait for the key to expire
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Now test the GET command
        let cmd = Get;
        let get_args = vec![RespType::BulkString(Option::from("temp_key".to_string()))];

        let result = cmd.execute(&get_args, &mut state).unwrap();

        assert_eq!(result, RespType::BulkString(None));
    }
}
