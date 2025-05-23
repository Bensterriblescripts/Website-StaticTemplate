use crate::webservice_types::{
    StaticVariables,
    AppState,

    IndexTemplate,
    PhotosTemplate,
    ContactTemplate,
};

// External Crates
use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router, 
};
use axum_extra::extract::CookieJar;

/* Start Webservice */
pub async fn start(static_variables: StaticVariables) {
    println!("\n:: Web Service ::");

    let address = static_variables.website_domain.clone();
    let state = AppState {
        static_variables,
    };

    // Router
    let app = Router::new() 
        .route("/", get(index))
        .route("/photos", get(photos))
        .route("/contact", get(contact))

        .route("/health", get(health_check))
        .nest_service("/static", tower_http::services::ServeDir::new("templates/static"))

        .with_state(state);

    let listener = match tokio::net::TcpListener::bind(address).await {
        Ok(listener) => {
            println!("Passed :: Web Service");

            /* Start */
            println!("\nServer Started...");
            listener
        },
        Err(error) => {
            println!("Error :: Web Service :: {error}");
            return;
        }
    };

    axum::serve(listener, app).await.unwrap();
}

/* Cookies */
// fn cookie_getlastpage(jar: CookieJar) -> (String, CookieJar) {
//     match jar.get("lastpage") {
//         Some(cookie_lastpage) => {
//             let lastpage = cookie_lastpage.value().parse::<String>().unwrap();
//             (lastpage, jar)
//         }
//         None => ("".to_string(), jar)
//     }
// }

/* Index */
async fn index(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let template = IndexTemplate {
        static_variables: state.static_variables,
        pagename: "Home".to_string(),
    };

    (jar, Html(template.render().unwrap())).into_response()
}
/* Photos */
async fn photos(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let template = PhotosTemplate {
        static_variables: state.static_variables,
        pagename: "Photos".to_string(),
    };
    (jar, Html(template.render().unwrap())).into_response()
}
/* Contact */
async fn contact(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let template = ContactTemplate {
        static_variables: state.static_variables,
        pagename: "Contact".to_string(),
    };
    (jar, Html(template.render().unwrap())).into_response()
}

/* Health Check */
async fn health_check() -> &'static str {
    "OK"
}
