use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct ExportsInfo {
  pub exports: HashMap<String, ExportInfo>,
}

#[derive(Debug, Default, Clone)]
pub struct ExportInfo {
  pub name: String,
  pub can_mangle: bool,
  pub used: bool,
  pub used_info: bool,
  pub provided_info: bool,
}

#[derive(Debug, Default, Clone)]
pub struct ProvidedInfo {
  pub provided: bool,
}
