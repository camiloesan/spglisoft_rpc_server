use crate::dal::data_access;
use crate::data_manager::devs_by_project_id_list::DevInfo;
use crate::data_manager::{DeveloperCredentials, ProjectManagerCredentials};

use mysql::{params, prelude::Queryable, Row};

pub fn get_developer_info(matricula: String, contrasena: String) -> DeveloperCredentials {
    let query = "SELECT * FROM desarrollador 
        WHERE matricula = :matricula AND contrasena = :contrasena";
    let mut conn = data_access::get_connection();
    let mut dev_credentials = DeveloperCredentials::default();

    let _result = conn
        .exec_map(
            &query,
            params! { "matricula" => matricula,
            "contrasena" => contrasena },
            |mut row: Row| {
                dev_credentials = DeveloperCredentials {
                    id_desarrollador: row.take("id_desarrollador").unwrap(),
                    nombre: row.take("nombre").unwrap(),
                    apellido_paterno: row.take("apellido_paterno").unwrap(),
                    apellido_materno: row.take("apellido_materno").unwrap(),
                    matricula: row.take("matricula").unwrap(),
                    id_proyecto: row.take("id_proyecto").unwrap(),
                    semestre: row.take("semestre").unwrap(),
                    id_experiencia_educativa: row.take("id_experiencia_educativa").unwrap(),
                };
            },
        )
        .expect("failed to get developer information");

    dev_credentials
}

pub fn get_project_manager_info(
    numero_personal: String,
    contrasena: String,
) -> ProjectManagerCredentials {
    let query = "SELECT * FROM representante_proyecto 
        WHERE numero_personal = :numero_personal AND contrasena = :contrasena";

    let mut conn = data_access::get_connection();
    let mut manager_credentials = ProjectManagerCredentials::default();

    let _result = conn
        .exec_map(
            &query,
            params! { "numero_personal" => numero_personal,
            "contrasena" => contrasena },
            |mut row: Row| {
                manager_credentials = ProjectManagerCredentials {
                    id_representante: row.take("id_representante").unwrap(),
                    nombre: row.take("nombre").unwrap(),
                    apellido_paterno: row.take("apellido_paterno").unwrap(),
                    apellido_materno: row.take("apellido_materno").unwrap(),
                };
            },
        )
        .expect("failed to get pm information");

    manager_credentials
}

pub fn get_devs_by_project_id(project_id: i32) -> Vec<DevInfo> {
    let query = "SELECT id_desarrollador, nombre, apellido_paterno, 
        apellido_materno, matricula, semestre 
        FROM desarrollador WHERE id_proyecto = :project_id";

    let mut dev_info_vec: Vec<DevInfo> = Vec::new();
    let mut conn = data_access::get_connection();
    let _result = conn
        .exec_map(
            query,
            params! { "project_id" => project_id },
            |mut row: Row| {
                let info = DevInfo {
                    id_desarrollador: row.take("id_desarrollador").unwrap(),
                    nombre: row.take("nombre").unwrap(),
                    apellido_paterno: row.take("apellido_paterno").unwrap(),
                    apellido_materno: row.take("apellido_materno").unwrap(),
                    matricula: row.take("matricula").unwrap(),
                    semestre: row.take("semestre").unwrap(),
                };
                dev_info_vec.push(info);
            },
        )
        .expect("failed to get devs by id");

    dev_info_vec
}

pub fn get_devs_without_project() -> Vec<DevInfo> {
    let query = "SELECT id_desarrollador, nombre, apellido_paterno, 
        apellido_materno, matricula,semestre 
        FROM desarrollador WHERE id_proyecto IS NULL";

    let mut dev_info_vec: Vec<DevInfo> = Vec::new();
    let mut conn = data_access::get_connection();
    let _result = conn
        .query_map(query, |mut row: Row| {
            let info = DevInfo {
                id_desarrollador: row.take("id_desarrollador").unwrap(),
                nombre: row.take("nombre").unwrap(),
                apellido_paterno: row.take("apellido_paterno").unwrap(),
                apellido_materno: row.take("apellido_materno").unwrap(),
                matricula: row.take("matricula").unwrap(),
                semestre: row.take("semestre").unwrap(),
            };
            dev_info_vec.push(info);
        })
        .expect("failed to get devs by id");

    dev_info_vec
}

pub fn delete_dev_from_project(id_developer: i32) -> i32 {
    let query = "UPDATE desarrollador 
        SET id_proyecto = NULL 
        WHERE id_desarrollador = :id_developer";

    let mut conn = data_access::get_connection();
    let rows_affected = conn.exec_iter(&query, params! { "id_developer" => id_developer})
        .unwrap()
        .affected_rows();

    rows_affected as i32
}

pub fn add_dev_to_project(id_project: i32, id_developer: i32) -> i32 {
    let query = "UPDATE desarrollador 
        SET id_proyecto = :id_project 
        WHERE id_desarrollador = :id_developer";

    let mut conn = data_access::get_connection();
    let rows_affected = conn.exec_iter(
        &query, 
        params! { "id_project" => id_project, "id_developer" => id_developer})
        .unwrap()
        .affected_rows();
    
    rows_affected as i32
}

pub fn assign_activity(id_activity: i32 , id_developer: i32) -> i32 {
    let query = "UPDATE actividad 
        SET id_desarrollador = :id_developer 
        WHERE id_actividad = :id_activity";

    let mut conn = data_access::get_connection();
    let rows_affected = conn.exec_iter(
        &query, 
        params! { "id_developer" => id_developer, "id_activity" => id_activity})
        .unwrap()
        .affected_rows();
    
    rows_affected as i32
}