use clap::{Parser, Subcommand};
use rpassword::prompt_password;
use uuid::Uuid;

// Importa los módulos de tu arquitectura
mod application;
mod domain;
mod infrastructure;

use crate::application::use_cases::password_use_case::PasswordUseCase;
use crate::domain::entities::password_entity::PasswordEntity;

/// Un administrador de contraseñas simple implementado con arquitectura limpia en Rust
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Añade una nueva contraseña
    Add {
        /// El nombre del sitio o servicio
        #[arg(short, long)]
        site: String,

        /// El nombre de usuario para el servicio
        #[arg(short, long)]
        username: String,

        /// El email asociado (opcional)
        #[arg(short, long)]
        email: Option<String>,
    },
    /// Obtiene la contraseña de un servicio
    Get {
        /// El nombre del sitio o servicio
        #[arg(short, long)]
        site: String,

        /// El nombre de usuario para el servicio
        #[arg(short, long)]
        username: String,
    },
    /// Lista todos los servicios y usuarios almacenados
    List,
    /// Elimina una contraseña
    Delete {
        /// El nombre del sitio o servicio a eliminar
        #[arg(short, long)]
        site: String,

        /// El nombre de usuario del servicio a eliminar
        #[arg(short, long)]
        username: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // --- Composición de Dependencias (El "Composition Root") ---
    // Aquí es donde "conectas" las implementaciones concretas.
    // Creamos la implementación concreta del repositorio.
    let password_repository =
        Box::new(infrastructure::persistence::password_repository::PasswordRepository::new("passwords.json"));

    // Inyectamos el repositorio en el caso de uso.
    let password_use_case = PasswordUseCase::new(password_repository);
    // --- Fin de la Composición ---


    match cli.command {
        Commands::Add { site, username, email } => {
            println!("Añadiendo contraseña para {} en {}", username, site);
            let password = prompt_password("Por favor, introduce la contraseña: ").unwrap();
            
            let entity = PasswordEntity::new(
                Uuid::new_v4().to_string(),
                site.clone(), // page_url
                site,         // page_name
                username,
                email.unwrap_or_default(), // email
                password,
            );

            match password_use_case.add_password(&entity) {
                Ok(_) => println!("¡Contraseña añadida con éxito!"),
                Err(e) => eprintln!("Error al añadir la contraseña: {}", e),
            }
        }
        Commands::Get { site, username } => {
            match password_use_case.get_password(&site, &username) {
                Ok(Some(entity)) => {
                    println!("Contraseña para {} en {}: {}", entity.username, entity.page_name, entity.password);
                }
                Ok(None) => {
                    println!("No se encontró ninguna contraseña para {} en {}", username, site);
                }
                Err(e) => eprintln!("Error al obtener la contraseña: {}", e),
            }
        }
        Commands::List => {
            match password_use_case.get_all_passwords() {
                Ok(entries) => {
                    if entries.is_empty() {
                        println!("No hay contraseñas almacenadas.");
                    } else {
                        println!("Contraseñas almacenadas:");
                        println!("{:<20} | {:<20}", "Sitio", "Usuario");
                        println!("{}", "-".repeat(43));
                        for entry in entries {
                            println!("{:<20} | {:<20}", entry.page_name, entry.username);
                       }
                    }
                }
                Err(e) => eprintln!("Error al listar las contraseñas: {}", e),
            }
        }
        Commands::Delete { site, username } => {
            match password_use_case.delete_password(&site, &username) {
                Ok(_) => println!("¡Contraseña para {} en {} eliminada con éxito!", username, site),
                Err(e) => eprintln!("Error al eliminar la contraseña: {}", e),
            }
        }
    }
}