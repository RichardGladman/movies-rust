pub struct MovieModel {
    title: String,
    format: String,
    certificate: String,
    rating: u32,
    running_time: u32
}

impl MovieModel {
    pub fn new(title: String, format: String, certificate: String, rating: u32, running_time: u32) -> MovieModel {
        return MovieModel {
            title, format, certificate, rating, running_time
        }
    }
}