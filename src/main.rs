use rust_db_manager_core::{
    commons::configuration::configuration::Configuration,
    domain::connection_data::ConnectionData,
    infrastructure::{db_service::DBService, repository::e_db_repository::EDBRepository},
};

use rust_db_manager_tui::infrastructure::manager::data_base::manager_database::ManagerDatabase;


#[tokio::main]
async fn main() {
    let _ = Configuration::initialize();

    let key = String::from("MONGO_DB");
    let data = ConnectionData::new(
        EDBRepository::MongoDB,
        String::from("mongodb://root:example@localhost:27017"),
    );
    let serv = DBService::new(key.clone(), String::from("ADMIN"), data);

    Configuration::push_service(key.clone(), serv);

    let serv = Configuration::find_service(key).unwrap();
    let service = serv.instance().await.expect("Initialize error.");

    let mut terminal = ManagerDatabase::new(service);
    terminal.launch().await;

    println!("rust-db-manager!");
}
