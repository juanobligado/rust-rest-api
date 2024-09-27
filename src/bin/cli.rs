use clap::{Command, Arg, value_parser};
use rocket::http::hyper::body::HttpBody;

#[tokio::main]
async fn main() {
    let matches = Command::new("cargo_package_manager")
        .about("My super CLI")
        .author("Me")
        .arg_required_else_help(true)
        .subcommand(
            {
                Command::new("users")
                    .about("User Management")
                    .arg_required_else_help(true)
                    .subcommand(
                        Command::new("list")
                            .about("List existing users")
                            .arg_required_else_help(false)
                    )
                    .subcommand(
                        Command::new("create")
                            .about("Add a user")
                            .arg_required_else_help(true)
                            .arg(Arg::new("username").required(true))
                            .arg(Arg::new("password").required(true))
                            .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))
                    )
                    .subcommand(
                        Command::new("delete")
                            .about("Delete a user by id")
                            .arg_required_else_help(true)
                            .arg(Arg::new("id")
                            .required(true)
                            .value_parser(value_parser!(i32)))
                    )
            }
        ).get_matches();
    match matches.subcommand() {
        Some(("users", users_matches)) => {
            match users_matches.subcommand() {
                Some(("list", _)) => {
                    println!("Listing users....");
                    backend_crate::commands::list_users().await;
                }
                Some(("create", create_matches)) => {
                    let username = create_matches.get_one::<String>("username").unwrap().to_owned();
                    let password = create_matches.get_one::<String>("password").unwrap().to_owned();
                    let roles = create_matches.get_many::<String>("roles").unwrap().map(|v| v.to_owned()).collect();
                    backend_crate::commands::create_user(&username, &password, roles).await;
                }
                Some(("delete", delete_matches)) => {
                    backend_crate::commands::delete_user(
                        delete_matches.get_one::<i32>("id").unwrap().to_owned()
                    ).await;
                }
                _=> {}
            }
        }
        _ => unreachable!()
    }

}
