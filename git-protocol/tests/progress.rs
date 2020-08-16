mod decode {
    use bstr::ByteSlice;
    use git_protocol::progress;

    #[test]
    fn a_message_we_dont_understand() {
        assert_eq!(
            progress::Remote::from_bytes(b"something that might be progress: but is not."),
            progress::Remote {
                action: b"something that might be progress: but is not.".as_bstr(),
                percent: None,
                step: None,
                max: None
            }
        )
    }

    #[test]
    fn enumerating_just_with_count() {
        assert_eq!(
            progress::Remote::from_bytes(b"Enumerating objects: 10, done."),
            progress::Remote {
                action: b"Enumerating objects".as_bstr(),
                percent: None,
                step: Some(10),
                max: None
            }
        )
    }
}