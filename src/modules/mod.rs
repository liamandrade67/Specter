pub mod recon;
pub mod web;
pub mod crypto;
pub mod utilities;

pub fn categories() -> Vec<&'static str> {
    vec!["Recon", "Web", "Crypto", "Utilities"]
}

pub fn tools_for_category(index: usize) -> Vec<&'static str> {
    match index {
        0 => recon::tools(),
        1 => web::tools(),
        2 => crypto::tools(),
        3 => utilities::tools(),
        _ => vec![],
    }
}
