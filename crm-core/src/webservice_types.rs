use askama::Template;

// Static Variables
#[derive(Clone)]
pub struct StaticVariables {
    pub business_name: String,
    pub website_domain: String,
}
// State
#[derive(Clone)]
pub struct AppState {
    pub static_variables: StaticVariables,
}

// Page Templates
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub pagename: String,   
    pub static_variables: StaticVariables,
}
#[derive(Template)]
#[template(path = "photos.html")]
pub struct PhotosTemplate {
    pub pagename: String,
    pub static_variables: StaticVariables,
}
#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate {
    pub pagename: String,
    pub static_variables: StaticVariables,
}