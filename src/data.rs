pub mod data {
    const GEO_DEG_PRECISION: f64 = 100_000.0;

    /// Type that represents a latitude or longitude in degrees. It holds coordinates with a fixed precision of up to 5 decimal places.
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub struct GeoDeg(f32);

    impl GeoDeg {
        /// Returns the value of the GeoDeg as f64.
        pub fn as_f64(self) -> f64 {
            self.0 as f64
        }

        /// Returns the value of the GeoDeg as f32.
        pub fn as_f32(self) -> f32 {
            self.0
        }
    }

    impl From<f64> for GeoDeg {
        /// Creates a new GeoDeg from an f64.
        fn from(value: f64) -> Self {
            Self((value * GEO_DEG_PRECISION).round() as f32 / GEO_DEG_PRECISION as f32)
        }
    }

    impl From<f32> for GeoDeg {
        /// Creates a new GeoDeg from an f32.
        fn from(value: f32) -> Self {
            Self((value * GEO_DEG_PRECISION as f32).round() / GEO_DEG_PRECISION as f32)
        }
    }

    impl std::ops::Add for GeoDeg {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            Self::from(self.0 + other.0)
        }
    }

    impl std::ops::Sub for GeoDeg {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            Self::from(self.0 - other.0)
        }
    }

    impl std::ops::Mul<f32> for GeoDeg {
        type Output = Self;
        fn mul(self, scalar: f32) -> Self {
            Self::from(self.0 * scalar)
        }
    }

    impl std::ops::Div<f32> for GeoDeg {
        type Output = Self;
        fn div(self, scalar: f32) -> Self {
            Self::from(self.0 / scalar)
        }
    }

    impl std::fmt::Display for GeoDeg {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:.5}", self.0)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Site {
        pub id: i32,
        pub lon: GeoDeg,
        pub lat: GeoDeg,
    }
}
