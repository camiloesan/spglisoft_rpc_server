mod dal;

use dal::{user, project};
use data_manager::data_manager_server::{DataManager, DataManagerServer};
use data_manager::{
    ActDevIdRequest, DeleteDevRequest, DevProjIdRequest, DeveloperCredentials, DeveloperRequest,
    DevsByProjectIdList, DevsByProjectIdRequest, NoParams, ProjectManagerCredentials,
    ProjectManagerRequest, RowsAffected, ManagerIdRequest, ProjectList, DevIdRequest, ProjectInfo,
};
use tonic::{transport::Server, Request, Response, Status};

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
        let request_data = request.into_parts();
        let response =
            user::get_developer_info(request_data.2.matricula, request_data.2.contrasena);
        Ok(Response::new(response))
    }

    async fn get_project_manager(
        &self,
        request: Request<ProjectManagerRequest>,
    ) -> Result<Response<ProjectManagerCredentials>, Status> {
        let request_data = request.into_parts();
        let response = user::get_project_manager_info(
            request_data.2.numero_personal,
            request_data.2.contrasena,
        );
        Ok(Response::new(response))
    }

    async fn get_devs_by_project_id(
        &self,
        request: Request<DevsByProjectIdRequest>,
    ) -> Result<Response<DevsByProjectIdList>, Status> {
        let request_data = request.into_parts();
        let response = DevsByProjectIdList {
            devs: user::get_devs_by_project_id(request_data.2.id_proyecto),
        };
        Ok(Response::new(response))
    }

    async fn get_devs_without_project(
        &self,
        _request: Request<NoParams>,
    ) -> Result<Response<DevsByProjectIdList>, Status> {
        let response = DevsByProjectIdList {
            devs: user::get_devs_without_project(),
        };
        Ok(Response::new(response))
    }

    async fn delete_dev_from_project(
        &self,
        request: Request<DeleteDevRequest>,
    ) -> Result<Response<RowsAffected>, Status> {
        let request_data = request.into_parts();
        let response = RowsAffected {
            rows_affected: user::delete_dev_from_project(request_data.2.id_desarrollador),
        };
        Ok(Response::new(response))
    }

    async fn add_dev_to_project(
        &self,
        request: Request<DevProjIdRequest>,
    ) -> Result<Response<RowsAffected>, Status> {
        let request_data = request.into_parts();
        let response = RowsAffected {
            rows_affected: user::add_dev_to_project(
                request_data.2.id_proyecto,
                request_data.2.id_desarrollador,
            ),
        };
        Ok(Response::new(response))
    }

    async fn assign_activity(
        &self,
        request: Request<ActDevIdRequest>,
    ) -> Result<Response<RowsAffected>, Status> {
        let request_data = request.into_parts();
        let response = RowsAffected {
            rows_affected: user::assign_activity(
                request_data.2.id_actividad,
                request_data.2.id_desarrollador,
            ),
        };
        Ok(Response::new(response))
    }

    async fn get_projects_by_manager(
        &self,
        request: Request<ManagerIdRequest>,
    ) -> Result<Response<ProjectList>, Status> {
        let request_data = request.into_parts();
        let response = ProjectList {
            proyectos: project::get_projects_by_manager(request_data.2.id_representante),
        };
        Ok(Response::new(response))
    }

    async fn get_project_by_dev(
        &self,
        request: Request<DevIdRequest>,
    ) -> Result<Response<ProjectInfo>, Status> {
        let request_data = request.into_parts();
        let response = project::get_project_by_dev(request_data.2.id_desarrollador);
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
