pub struct LatitudeLongitude {
    pub lat: f32,
    pub long: f32,
}

pub fn get_hawaii_location() -> LatitudeLongitude {
    LatitudeLongitude{
        lat: 19.896,
        long: 155.583,
    }
} 