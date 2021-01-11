#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

const SECONDS_IN_EARTH_YEAR: u64 = 31557600;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            earth_years: s as f64 / SECONDS_IN_EARTH_YEAR as f64,
        }
    }
}

pub trait Planet {
    const ORBIT: f64;
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / Self::ORBIT
    }
}

macro_rules! create_planet_struct {
    ($t:ident, orbit = $orbit:expr) => {
        pub struct $t;
        impl Planet for $t {
            const ORBIT: f64 = $orbit;
        }
    };
}

create_planet_struct!(Mercury, orbit = 0.2408467);
create_planet_struct!(Venus, orbit = 0.61519726);
create_planet_struct!(Mars, orbit = 1.8808158);
create_planet_struct!(Jupiter, orbit = 11.862615);
create_planet_struct!(Saturn, orbit = 29.447498);
create_planet_struct!(Uranus, orbit = 84.016846);
create_planet_struct!(Neptune, orbit = 164.79132);
create_planet_struct!(Earth, orbit = 1.0);
