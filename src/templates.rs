
#[derive(rinja::Template)]
#[template(path = "index.html")]
pub struct Index {}

pub struct Dog<'a> {
    pub id: i64,
    pub name: &'a str,
    pub bio: &'a str,
    pub breed: &'a str,
    pub image: &'a str,
    pub account: i64,
    // pub birth_date: &'a str,
}

pub struct Thanks {
    pub dog_index: usize,
    pub dog_id: i64,
}

#[derive(rinja::Template)]
#[template(path = "dogs.html")]
pub struct Dogs<'a> {
    pub shelter: usize,
    pub dogs: &'a[Dog<'a>],
    pub thanks: Option<Thanks>,
}

#[derive(rinja::Template)]
#[template(path = "dog_viewer.html")]
pub struct DogViewer<'a> {
    pub shelter: usize,
    pub dog: &'a Dog<'a>,
    pub appreciative: bool,
}
