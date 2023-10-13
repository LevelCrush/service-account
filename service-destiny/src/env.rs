use levelcrush::project_str;
use lib_destiny::env::{AppVariable, Env};

/// baked in compile time configuration for defaults. Will be used first
pub const BAKED_CONFIG: &str = project_str!("env.compile.json");

/* commenting for now,  could of sworn i had used this at one point but cant remember where or why
pub fn exists(app_var: AppVariable) -> bool {
    std::env::var::<&'static str>(app_var.into()).is_ok()
}
*/

pub fn load() -> Env {
    Env::load(BAKED_CONFIG)
}
