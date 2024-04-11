pub mod messenger {
    trait Logger {
        fn warning(&self, msg: &str);
        fn info(&self, msg: &str);
        fn error(&self, msg: &str);
    }
    struct Tracker {
        logger: dyn Logger,
        value: usize,
        max: usize
    }
    impl Tracker {
        fn new(logger: &dyn Logger, max: usize) -> Self {
            Self {
                logger,
                value: 0,
                max
            }
        }

        fn set_value
    }
}
