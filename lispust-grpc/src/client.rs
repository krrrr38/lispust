use lispust::lispust_client::LispustClient;
use lispust::RunRequest;
use std::env;

pub mod lispust {
    tonic::include_proto!("lispust");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("client require one args")
    }
    let command = &args[1];

    let mut client = LispustClient::connect("lispust-http://[::1]:50051").await?;

    let req = tonic::Request::new(RunRequest {
        message: command.to_string(),
    });

    let res = client.run(req).await?;
    println!("{:?}", res);
    println!("message = {:?}", res.into_inner().message);

    Ok(())
}
