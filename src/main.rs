use aws_sdk_ecs_demo::constants::*;
use aws_sdk_ecs_demo::{get_ecs_client, run_ecs_task};
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let subnets = SUBNETS.iter().map(|x| x.to_string()).collect();
    let security_groups = SECURITY_GROUPS.iter().map(|x| x.to_string()).collect();
    let client = get_ecs_client(REGION.to_string()).await;
    let output = run_ecs_task(&client, CLUSTER, TASK_NAME, CONTAINER_NAME, Some(subnets), Some(security_groups), FOO).await?;
    println!("{:?}", output);
    Ok(())
}
