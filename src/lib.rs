use aws_config::{BehaviorVersion, Region};
use aws_sdk_ecs::{operation::run_task::RunTaskOutput, types::{AssignPublicIp, AwsVpcConfiguration, ContainerOverride, KeyValuePair, LaunchType, NetworkConfiguration, TaskOverride}, Client};
use color_eyre::Result;

pub mod constants;

pub async fn get_ecs_client(region: String) -> Client {
    let region = Region::new(region);
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region)
        .load()
        .await;
    Client::new(&sdk_config)
}

pub async fn run_ecs_task(
    client: &Client, 
    cluster: &str,
    task_definition: &str,
    container: &str,
    subnets: Option<Vec<String>>,
    security_groups: Option<Vec<String>>,
    foo: &str,
) -> Result<RunTaskOutput> {
    let foo = KeyValuePair::builder().name("FOO").value(foo).build();
    let overrides = TaskOverride::builder()
        .container_overrides(
            ContainerOverride::builder()
                .name(container)
                .environment(foo)
                .build(),
        )
        .build();

    let network_configuration = NetworkConfiguration::builder()
    .awsvpc_configuration(
        AwsVpcConfiguration::builder()
            .set_subnets(subnets)
            .set_security_groups(security_groups)
            .assign_public_ip(AssignPublicIp::Disabled) 
            .build()?,
    )
    .build();

    let run_task_builder = client.run_task();
    let run_task_builder = run_task_builder
        .cluster(cluster)
        .task_definition(task_definition)
        .launch_type(LaunchType::Fargate)
        .network_configuration(network_configuration)
        .overrides(overrides);

    let output = run_task_builder.send().await?;
    Ok(output)
}