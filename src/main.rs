use aws_config;
use aws_sdk_ssm as ssm;
use ssm::client;

#[tokio::main]
async fn main() -> Result<(), ssm::Error> {
    let config = aws_config::from_env().region("us-east-1").load().await;
    let client = ssm::Client::new(&config);

    let document_name = "AWS-RunShellScript";
    let instance_id = "i-055990c0ae2087747";

    let command_parameters = Some(std::collections::HashMap::from_iter(vec![
        ("commands".to_owned(), vec!["echo hello".to_owned()]),
        ("executionTimeout".to_owned(), vec!["3600".to_owned()]),
    ]));
    let command = client
        .send_command()
        .document_name(document_name)
        .set_instance_ids(Some(vec![instance_id.to_owned()]))
        .set_parameters(command_parameters)
        .comment("Run a shell script on the instance")
        .send()
        .await?;

    let command_id = command.command().unwrap();

    println!(
        "Running command {:?}",
        command_id.command_id().unwrap_or_default()
    );

    let output = client
        .get_command_invocation()
        .command_id(command_id.command_id().unwrap_or_default())
        .instance_id(instance_id)
        .send()
        .await?;

    //wait 5 seconds

    let duration = std::time::Duration::from_secs(5);

    let _ = tokio::time::sleep(duration).await;

    println!(
        "Output: {:?}",
        output.standard_output_content().unwrap_or_default()
    );
    Ok(())
}
