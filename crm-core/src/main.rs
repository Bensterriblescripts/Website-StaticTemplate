mod webservice;
mod webservice_types;
use crate::webservice_types::{StaticVariables, NavbarMenu, NavbarTemplate};

#[tokio::main]
async fn main() {

    let static_variables = StaticVariables {
        business_name: "Paradise Coffee".to_string(),
        website_domain: "127.0.0.1:3000".to_string(),
    };
    let navbar_template: NavbarTemplate = build_navbar_menus().await;
    webservice::start(navbar_template, static_variables).await;
}

async fn build_navbar_menus() -> NavbarTemplate {
    // Retrieve Menus
    let mut navbar_menus: Vec<NavbarMenu> = Vec::new();
    navbar_menus.push(NavbarMenu {
        label: "Home".to_string(),
        path: "/".to_string(),
    });
    navbar_menus.push(NavbarMenu {
        label: "About".to_string(),
        path: "/about".to_string(),
    });

    NavbarTemplate { menus: navbar_menus}
}
