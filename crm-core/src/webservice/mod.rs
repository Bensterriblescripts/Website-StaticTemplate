use crate::webservice_types::{
    StaticVariables,
    AppState,
    NavbarTemplate,

    IndexTemplate,
    AboutTemplate,
};

// External Crates
use askama::Template;
use axum::{
    extract::State,
    http::HeaderValue,
    response::{Html, IntoResponse},
    routing::get,
    Router, 
};
use axum_extra::extract::CookieJar;
use tower_http::{set_header::SetResponseHeaderLayer, cors::CorsLayer};

/* Start Webservice */
pub async fn start(navbar_template: NavbarTemplate, static_variables: StaticVariables) {
    println!("\n:: Web Service ::");

    let address = static_variables.website_domain.clone();
    let state = AppState {
        navbar: navbar_template, 
        static_variables,
    };

    // Allowed Origins
    let allowed_origins = [
        HeaderValue::from_static("http://localhost:3000"),
        HeaderValue::from_str(&format!("https://{}", address)).unwrap(),
    ];
    let cors = CorsLayer::new()
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE])
        .allow_origin(allowed_origins.to_owned());

    // Router
    let app = Router::new() 
        .route("/", get(index))
        .route("/health", get(health_check))
        .route("/about", get(about))

        .nest_service("/static", tower_http::services::ServeDir::new("templates/static"))
        .nest_service("/store", tower_http::services::ServeDir::new("store"))
        // Headers/Security
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static("default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline';"), // This will likely require a partial navbar overhaul to do correctly
        ))
        .layer(cors)

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
        navbar: state.navbar,
        pagename: "Home".to_string(),
    };

    (jar, Html(template.render().unwrap())).into_response()
}
/* About */
async fn about(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let template = AboutTemplate {
        static_variables: state.static_variables,
        navbar: state.navbar,
        pagename: "About".to_string(),
    };
    (jar, Html(template.render().unwrap())).into_response()
}

/* Health Check */
async fn health_check() -> &'static str {
    "OK"
}