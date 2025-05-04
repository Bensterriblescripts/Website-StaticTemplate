use askama::Template;

#[derive(Clone)]
pub struct StaticVariables {
    pub business_name: String,
    pub website_domain: String,
}

// Navigation Menu
#[derive(Clone)]
pub struct NavbarTemplate {
    pub menus: Vec<NavbarMenu>,
}
#[derive(Clone)]
pub struct NavbarMenu {
    pub label: String,
    pub path: String,
}

// Page Templates
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub navbar: NavbarTemplate,
    pub pagename: String,   
    pub static_variables: StaticVariables,
}
#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    pub navbar: NavbarTemplate,
    pub pagename: String,
    pub static_variables: StaticVariables,
}

// States
#[derive(Clone)]
pub struct AppState {
    pub navbar: NavbarTemplate,
    pub static_variables: StaticVariables,
}