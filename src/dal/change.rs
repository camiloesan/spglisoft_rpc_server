use mysql::{prelude::Queryable, Row};

use crate::{dal::data_access, data_manager::ChangeTypeInfo};

pub fn get_change_types() -> Vec<ChangeTypeInfo> {
    let query = "select * from tipo_cambio";

    let mut change_type_vec: Vec<ChangeTypeInfo> = Vec::default();
    let mut conn = data_access::get_connection();
    let _result = conn
        .query_map(
            query,
            |mut row: Row| {
        let change_type = ChangeTypeInfo {
            id_tipo_cambio: row.take("id_tipo_cambio").unwrap(),
            tipo_cambio: row.take("tipo_cambio").unwrap(),
        };
        change_type_vec.push(change_type);
    }).expect("failed to get change types");

    change_type_vec
}