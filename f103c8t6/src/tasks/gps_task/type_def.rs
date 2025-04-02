#[derive(Debug, defmt::Format)]
pub struct GpsRMC {
    pub status: char, // A=active, V=void
    pub latitude: f32,
    pub ulatitude: char, // N=north, S=south
    pub longitude: f32,
    pub ulongitude: char, // E=east, W=west
    pub speed: f32,       // knots
}

impl GpsRMC {
    pub const fn new() -> GpsRMC {
        Self {
            status: 'V',
            latitude: 0.0,
            ulatitude: 'N',
            longitude: 0.0,
            ulongitude: 'E',
            speed: 0.0,
        }
    }
}
