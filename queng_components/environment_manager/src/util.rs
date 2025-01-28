use common_env::EnvironmentType;
use std::env;

/// Detects the environment type based on the value of the "ENV" environment variable.
///
/// # Arguments
///
/// * `dbg`: If true, debug messages are printed.
///
/// # Returns
///
/// The detected `EnvironmentType`.
///
/// # Panics
///
/// If the "ENV" environment variable is not set or empty.
///
pub fn detect_env_type(dbg: bool) -> EnvironmentType {
    if dbg {
        println!("[EnvironmentManager]: Debug mode enabled");
    }

    let key = "ENV";
    // Check if the environment variable is set.
    assert!(
        env_is_set(key),
        "[EnvironmentManager]: ENV environment variable is not set or empty. Ensure ENV is set"
    );

    // If so, read its value and return the detected environment type.
    let env_type = match env::var(key) {
        Ok(val) => match val.as_str() {
            "CI" => EnvironmentType::CI,
            "CLUSTER" => EnvironmentType::CLUSTER,
            "LOCAL" => EnvironmentType::LOCAL,
            "UNKNOWN" => EnvironmentType::UNKNOWN,
            _ => EnvironmentType::UNKNOWN,
        },
        Err(e) => {
            eprintln!("Error: {e}");
            panic!(
                "[EnvironmentManager]: Failed to read ENV environment variable. Ensure ENV is set"
            );
        }
    };

    if dbg {
        println!(
            "[EnvironmentManager]: Detected environment type: {:?}",
            &env_type
        );
    }

    env_type
}

/// Checks if an environment variable is set and non-empty.
///
/// # Arguments
///
/// `key`: The name of the environment variable.
///
/// # Returns
///
/// `true` if the environment variable is set and non-empty, `false` otherwise.
fn env_is_set(key: &str) -> bool {
    match env::var(key) {
        Ok(s) => !s.is_empty(),

        _ => false,
    }
}
