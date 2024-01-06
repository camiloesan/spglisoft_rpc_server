use crate::dal::data_access;
use crate::data_manager::ProjectInfo;

use mysql::{params, prelude::Queryable, Row};

pub fn get_projects_by_manager(id_manager: i32) -> Vec<ProjectInfo> {
    let query = "SELECT p.id_proyecto, p.nombre_proyecto, p.descripcion, e.estado_proyecto 
    AS estado, DATE_FORMAT(p.fecha_inicio, '%d/%m/%Y') as fecha_inicio, 
    DATE_FORMAT(p.fecha_fin, '%d/%m/%Y') as fecha_fin, 
    r.nombre, r.apellido_paterno, r.apellido_materno 
    FROM proyecto p INNER JOIN estado_proyecto e ON p.estado_proyecto = e.id_estado_proyecto 
    INNER JOIN representante_proyecto r ON p.id_representante = r.id_representante 
    WHERE p.id_representante = :id_manager";

    let mut project_info_vec: Vec<ProjectInfo> = Vec::new();
    let mut conn = data_access::get_connection();
    let _result = conn
        .exec_map(
            query,
            params! { "id_manager" => id_manager },
            |mut row: Row| {
                let project = ProjectInfo {
                    id_proyecto: row.take("id_proyecto").unwrap(),
                    nombre_proyecto: row.take("nombre_proyecto").unwrap(),
                    descripcion: row.take("descripcion").unwrap(),
                    fecha_inicio: row.take("fecha_inicio").unwrap(),
                    fecha_fin: row.take("fecha_fin").unwrap(),
                    estado: row.take("estado").unwrap(),
                    nombre_representante: row.take("nombre").unwrap(),
                    apellido_paterno: row.take("apellido_paterno").unwrap(),
                    apellido_materno: row.take("apellido_materno").unwrap(),
                };
                project_info_vec.push(project);
            },
        )
        .expect("couldn't retrieve projects");

    project_info_vec
}

pub fn get_project_by_dev(id_developer: i32) -> ProjectInfo {
    let query = "SELECT p.id_proyecto, p.nombre_proyecto, p.descripcion, e.estado_proyecto 
    AS estado, DATE_FORMAT(p.fecha_inicio, '%d/%m/%Y') as fecha_inicio, 
    DATE_FORMAT(p.fecha_fin, '%d/%m/%Y') as fecha_fin, 
    r.nombre, r.apellido_paterno, r.apellido_materno 
    FROM proyecto p INNER JOIN estado_proyecto e ON p.estado_proyecto = e.id_estado_proyecto 
    INNER JOIN representante_proyecto r ON p.id_representante = r.id_representante 
    INNER JOIN desarrollador d on p.id_proyecto = d.id_proyecto 
    where id_desarrollador = :id_developer";

    let mut project: ProjectInfo = ProjectInfo::default();
    let mut conn = data_access::get_connection();
    let _result = conn
        .exec_map(
            query,
            params! { "id_developer" => id_developer },
            |mut row: Row| {
                project = ProjectInfo {
                    id_proyecto: row.take("id_proyecto").unwrap(),
                    nombre_proyecto: row.take("nombre_proyecto").unwrap(),
                    descripcion: row.take("descripcion").unwrap(),
                    fecha_inicio: row.take("fecha_inicio").unwrap(),
                    fecha_fin: row.take("fecha_fin").unwrap(),
                    estado: row.take("estado").unwrap(),
                    nombre_representante: row.take("nombre").unwrap(),
                    apellido_paterno: row.take("apellido_paterno").unwrap(),
                    apellido_materno: row.take("apellido_materno").unwrap(),
                };
            },
        )
        .expect("couldn't retrieve project");

    project
}
