mod data_access;

use tonic::{transport::Server, Request, Response, Status};
use data_manager::data_manager_server::{DataManager, DataManagerServer};
use data_manager::{DeveloperRequest, ProjectManagerRequest, 
    DeveloperCredentials, ProjectManagerCredentials};

pub mod data_manager {
    tonic::include_proto!("spglisoft");
}

#[derive(Debug, Default)]
pub struct DataService {}

#[tonic::async_trait]
impl DataManager for DataService {
    async fn get_developer(
        &self,
        request: Request<DeveloperRequest>,
    ) -> Result<Response<DeveloperCredentials>, Status> {
        println!("requested to get developer");

        let request_data = request.into_parts();
        let response 
            = data_access::data_access::get_developer_info(
                request_data.2.matricula, 
                request_data.2.contrasena,
            );
        Ok(Response::new(response))
    }

    async fn get_project_manager(
        &self,
        request: Request<ProjectManagerRequest>,
    ) -> Result<Response<ProjectManagerCredentials>, Status> {
        println!("requested to get manager");

        let request_data = request.into_parts();
        let response 
            = data_access::data_access::get_project_manager_info(
                request_data.2.numero_personal, 
                request_data.2.contrasena,
            );
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = DataService::default();

    Server::builder()
        .add_service(DataManagerServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}