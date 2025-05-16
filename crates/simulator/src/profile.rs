#[derive(Debug)]
pub struct Profile {
    pub name: String,
    pub bio: String,
    pub followers: usize,
    pub following: usize,
}

impl Profile {
    pub fn new(name: String, bio: String, followers: usize, following: usize) -> Self {
        Self {
            name,
            bio,
            followers,
            following,
        }
    }
}
