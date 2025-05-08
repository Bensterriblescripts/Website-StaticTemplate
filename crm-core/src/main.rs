mod webservice;
mod webservice_types;
use crate::webservice_types::StaticVariables;

#[tokio::main]
async fn main() {

    let static_variables = StaticVariables {
        business_name: "Paradise Coffee".to_string(),
        website_domain: "0.0.0.0:9050".to_string(),
    };
    webservice::start(static_variables).await;
}
