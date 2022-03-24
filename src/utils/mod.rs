pub fn requiredEnvVar(name: &str) -> String {
    return std::env::var(name)
        .expect(format!("Required environment variable not found: {name}").as_str());
}
