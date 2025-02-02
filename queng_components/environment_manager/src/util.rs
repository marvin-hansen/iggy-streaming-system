use common_env::EnvironmentType;
use common_platform::PlatformType;
use std::env;

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
pub(crate) fn detect_env_type(dbg: bool) -> EnvironmentType {
    if dbg {
        println!("[EnvironmentManager]: detect_env_type");
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

/// Detects the platform type based on the value of the "uname -v" command.
///
/// # Arguments
///
/// * `dbg`: If true, debug messages are printed.
///
/// # Returns
///
/// The detected `PlatformType`.
///
/// # Panics
///
/// If the "uname -v" command fails to execute.
///
pub(crate) fn detect_platform_type(dbg: bool) -> PlatformType {
    if dbg {
        println!("[EnvironmentManager]: detect_platform_type");
    }

    let output = std::process::Command::new("uname")
        .arg("-a")
        .output()
        .expect("Failed to execute uname -a command to detect host platform");

    let stdout = String::from_utf8_lossy(&output.stdout);

    if dbg {
        println!("[EnvironmentManager]: Detected host platform: {}", &stdout);
    }

    // Test if output is x86_64 and linux for PlatformType::LinuxX86_64
    // Test if output is aarch64 and linux for PlatformType::LinuxAarch64
    // Test if output is arm64 and Darwin for PlatformType::MacOSAarch64
    if stdout.contains("x86_64") && stdout.contains("Linux") {
        PlatformType::LinuxX86_64
    } else if stdout.contains("aarch64") && stdout.contains("Linux") {
        PlatformType::LinuxAarch64
    } else if stdout.contains("RELEASE_ARM64") && stdout.contains("Darwin") {
        PlatformType::MacOSAarch64
    } else {
        PlatformType::UnknownPlatform
    }
}
