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
    http::HeaderValue,
    response::{Html, IntoResponse},
    routing::get,
    Router, 
};
use axum_extra::extract::CookieJar;
use tower_http::{set_header::SetResponseHeaderLayer, cors::CorsLayer};

/* Start Webservice */
pub async fn start(static_variables: StaticVariables) {
    println!("\n:: Web Service ::");

    let address = static_variables.website_domain.clone();
    let state = AppState {
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
        .route("/photos", get(photos))
        .route("/contact", get(contact))

        .route("/health", get(health_check))
        .nest_service("/static", tower_http::services::ServeDir::new("templates/static"))

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