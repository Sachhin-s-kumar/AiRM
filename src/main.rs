const APP_NAME: &str = "Nexus";
const APP_VERSION: &str = "0.1.0";

fn get_version() -> &'static str {
    APP_VERSION
}

fn main() {
    println!("================================");
    println!("       {} v{}", APP_NAME, get_version());
    println!("================================");
    println!("AI Training Resource Manager");
    println!("Platform: Fedora Linux");
    println!("Status: Initializing...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_correct() {
        assert_eq!(get_version(), "0.1.0");
    }
}
