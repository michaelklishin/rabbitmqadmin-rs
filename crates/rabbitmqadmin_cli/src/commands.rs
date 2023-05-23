use clap::ArgMatches;

use rabbitmq_http_client::responses;
use rabbitmq_http_client::responses::Result as ClientResult;
use rabbitmq_http_client::blocking::Client as APIClient;

use crate::cli::SharedFlags;

pub fn list_nodes(general_args: &ArgMatches) -> ClientResult<Vec<responses::ClusterNode>> {
    let sf = SharedFlags::from_args(general_args);
    let endpoint = sf.endpoint();
    let rc =
        APIClient::new_with_basic_auth_credentials(&endpoint, &sf.username, Some(&sf.password));
    rc.list_nodes()
}

pub fn list_vhosts(general_args: &ArgMatches) -> ClientResult<Vec<responses::VirtualHost>> {
    let sf = SharedFlags::from_args(general_args);
    let endpoint = sf.endpoint();
    let rc =
        APIClient::new_with_basic_auth_credentials(&endpoint, &sf.username, Some(&sf.password));
    rc.list_vhosts()
}

pub fn list_users(general_args: &ArgMatches) -> ClientResult<Vec<responses::User>> {
    let sf = SharedFlags::from_args(general_args);
    let endpoint = sf.endpoint();
    let rc =
        APIClient::new_with_basic_auth_credentials(&endpoint, &sf.username, Some(&sf.password));
    rc.list_users()
}

pub fn list_connections(general_args: &ArgMatches) -> ClientResult<Vec<responses::Connection>> {
    let sf = SharedFlags::from_args(general_args);
    let endpoint = sf.endpoint();
    let rc =
        APIClient::new_with_basic_auth_credentials(&endpoint, &sf.username, Some(&sf.password));
    rc.list_connections()
}

pub fn list_channels(general_args: &ArgMatches) -> ClientResult<Vec<responses::Channel>> {
    let sf = SharedFlags::from_args(general_args);
    let endpoint = sf.endpoint();
    let rc =
        APIClient::new_with_basic_auth_credentials(&endpoint, &sf.username, Some(&sf.password));
    rc.list_channels()
}

pub fn list_consumers(general_args: &ArgMatches) -> ClientResult<Vec<responses::Consumer>> {
    let sf = SharedFlags::from_args(general_args);
    let endpoint = sf.endpoint();
    let rc =
        APIClient::new_with_basic_auth_credentials(&endpoint, &sf.username, Some(&sf.password));
    rc.list_consumers()
}

pub fn delete_vhost(general_args: &ArgMatches, command_args: &ArgMatches) -> ClientResult<()> {
    let sf = SharedFlags::from_args(general_args);
    // the flag is required
    let name = command_args.get_one::<String>("name").unwrap();
    let endpoint = sf.endpoint();
    let rc =
        APIClient::new_with_basic_auth_credentials(&endpoint, &sf.username, Some(&sf.password));
    rc.delete_vhost(&name)
}

pub fn delete_user(general_args: &ArgMatches, command_args: &ArgMatches) -> ClientResult<()> {
    let sf = SharedFlags::from_args(general_args);
    // the flag is required
    let name = command_args.get_one::<String>("name").unwrap();
    let endpoint = sf.endpoint();
    let rc =
        APIClient::new_with_basic_auth_credentials(&endpoint, &sf.username, Some(&sf.password));
    rc.delete_user(&name)
}

pub fn delete_queue(general_args: &ArgMatches, command_args: &ArgMatches) -> ClientResult<()> {
    let sf = SharedFlags::from_args(general_args);
    // the flag is required
    let name = command_args.get_one::<String>("name").unwrap();
    let endpoint = sf.endpoint();
    let rc =
        APIClient::new_with_basic_auth_credentials(&endpoint, &sf.username, Some(&sf.password));
    rc.delete_queue(&sf.virtual_host, &name)
}

pub fn purge_queue(general_args: &ArgMatches, command_args: &ArgMatches) -> ClientResult<()> {
    let sf = SharedFlags::from_args(general_args);
    // the flag is required
    let name = command_args.get_one::<String>("name").unwrap();
    let endpoint = sf.endpoint();
    let rc =
        APIClient::new_with_basic_auth_credentials(&endpoint, &sf.username, Some(&sf.password));
    rc.purge_queue(&sf.virtual_host, &name)
}
