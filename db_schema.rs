// Esquema inicial para la Base de Datos de Perfiles Inteligentes

mod usuarios {
    pub struct Usuario {
        pub id: String,
        pub nombre: String,
        pub edad: Option<u8>,
        pub preferencias: Vec<String>,
    }
}

mod historial {
    pub struct Visualizacion {
        pub id_usuario: String,
        pub id_contenido: String,
        pub tiempo_visto: f64,
        pub contexto_emocional: Option<String>,
    }
}

mod contenido {
    pub struct Contenido {
        pub id: String,
        pub titulo: String,
        pub genero: Vec<String>,
        pub calidad: String,
    }
}