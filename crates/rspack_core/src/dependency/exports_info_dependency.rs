use rspack_error::Result;
use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{Expr, Ident},
    utils::quote_ident,
  },
};

use crate::{
  create_javascript_visitor, CodeGeneratable, CodeGeneratableContext, CodeGeneratableResult,
  Dependency, JsAstPath, ModuleIdentifier,
};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct ExportsInfoDependency {
  pub export_name: Option<Vec<String>>,
  pub property: String,
  #[allow(unused)]
  pub ast_path: JsAstPath,
}

impl Dependency for ExportsInfoDependency {
  fn parent_module_identifier(&self) -> Option<&ModuleIdentifier> {
    None
  }
}

impl CodeGeneratable for ExportsInfoDependency {
  fn generate(
    &self,
    code_generatable_context: &mut CodeGeneratableContext,
  ) -> Result<CodeGeneratableResult> {
    let mut cgr = CodeGeneratableResult::default();

    // let mgm = code_generatable_context.compilation.module_graph.get_exports_info()
    let property = self.property.clone();
    let export_name = self.export_name.clone();
    cgr.visitors.push(
      create_javascript_visitor!(exact &self.ast_path, visit_mut_expr(n: &mut Expr) {
        if let Some(_) = export_name {
          if property.eq("used"){
            *n = Expr::Ident(quote_ident!(DUMMY_SP, "true"))
          } else if property.eq("useInfo")  {
            *n = Expr::Ident(quote_ident!(DUMMY_SP, "undefined"))
          } else if property.eq("provideInfo"){
            *n = Expr::Ident(quote_ident!(DUMMY_SP, "undefined"))
          } else {
            *n = Expr::Ident(quote_ident!(DUMMY_SP, "undefined"))
          }
        }

      }),
    );

    Ok(cgr)
  }
}

impl ExportsInfoDependency {
  pub fn new(export_name: Option<Vec<String>>, property: String, ast_path: JsAstPath) -> Self {
    Self {
      export_name,
      property,
      ast_path,
    }
  }
}
