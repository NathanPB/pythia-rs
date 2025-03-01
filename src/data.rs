pub mod data {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Site {
        pub id: i32,
        pub lon: f64,
        pub lat: f64,
    }
}
