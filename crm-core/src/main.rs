mod webservice;
mod webservice_types;
use crate::webservice_types::StaticVariables;

#[tokio::main]
async fn main() {

    let static_variables = StaticVariables {
        business_name: "Paradise Coffee".to_string(),
        website_domain: "127.0.0.1:3000".to_string(),
    };
    webservice::start(static_variables).await;
}