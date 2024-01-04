pub mod data_access {
    use mysql::{*, prelude::Queryable};
    use crate::data_manager::{ProjectManagerCredentials, DeveloperCredentials};

    fn get_connection() -> PooledConn {
        let url = "mysql://spglisoft_user:apple@localhost:3306/SPGLISOFT";
        let pool = Pool::new(url).expect("wrong");
        let conn = pool.get_conn().expect("wrong"); 
        conn
    }

    pub fn get_developer_info(matricula: String, contrasena: String)
         -> DeveloperCredentials {
        let query = "SELECT * FROM desarrollador 
            WHERE matricula = :matricula AND contrasena = :contrasena";
        let mut conn = get_connection();
        let mut dev_credentials = DeveloperCredentials::default();
        
        let _result = conn.exec_map(&query, 
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
            }
        ).expect("failed to get developer information");

        dev_credentials
    }

    pub fn get_project_manager_info(numero_personal: String, contrasena: String)
         -> ProjectManagerCredentials {
        let query = "SELECT * FROM representante_proyecto 
            WHERE numero_personal = :numero_personal AND contrasena = :contrasena";

        let mut conn = get_connection();
        let mut manager_credentials 
            = ProjectManagerCredentials::default();

        let _result = conn.exec_map(&query, 
            params! { "numero_personal" => numero_personal, 
            "contrasena" => contrasena }, 
            |mut row: Row| {
                manager_credentials = ProjectManagerCredentials {
                    id_representante: row.take("id_representante").unwrap(),
                    nombre: row.take("nombre").unwrap(),
                    apellido_paterno: row.take("apellido_paterno").unwrap(),
                    apellido_materno: row.take("apellido_materno").unwrap(),
                };
            }
        ).expect("failed to get pm information");

        manager_credentials
    }
}