use std::env;
use systemathics_apis::systemathics::apis::services::static_data::v1::static_data_service_client::StaticDataServiceClient;
use systemathics_apis::systemathics::apis::services::static_data::v1::AssetType::Equity;
use systemathics_apis::systemathics::apis::services::static_data::v1::StaticDataRequest;
use tonic::transport::Certificate;
use tonic::transport::ClientTlsConfig;

use tonic::{metadata::MetadataValue, transport::Channel, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pem = tokio::fs::read("examples/ganymede-cloud.pem").await?;
    let ca = Certificate::from_pem(pem);

    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("grpc.ganymede.cloud");

    let channel = Channel::from_static("https://grpc.ganymede.cloud")
        .tls_config(tls)?
        .connect()
        .await?;
    let args: Vec<String> = env::args().collect();
    let mut bearer = String::from("Bearer ");
    bearer.push_str(&args[1].to_owned());
    let token: MetadataValue<_> = bearer.parse()?;
    let mut client =
        StaticDataServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        });

    let request = tonic::Request::new(StaticDataRequest {
        name: String::from("Apple").into(),
        asset_type: Equity.into(),
        code: None,
        count: None,
        equity_sector: None,
        crypto_base_currency: None,
        crypto_quote_currency: None,
        exchange: None,
        future_category: None,
        future_contract: None,
        index: None,
        start: None,
        ticker: None,
    });

    let response = client.static_data(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
