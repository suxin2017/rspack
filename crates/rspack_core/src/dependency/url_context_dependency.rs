use rspack_error::Result;
use swc_core::{
  common::{Spanned, DUMMY_SP},
  ecma::{
    ast::{CallExpr, Callee, ExprOrSpread},
    atoms::JsWord,
    utils::{member_expr, quote_ident, quote_str, ExprFactory},
  },
};

use crate::{
  create_javascript_visitor, CodeGeneratable, CodeGeneratableContext, CodeGeneratableResult,
  ContextOptions, Dependency, DependencyId, ErrorSpan, JsAstPath, ModuleDependency,
  ModuleIdentifier, RuntimeGlobals,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UrlContextDependency {
  pub id: Option<DependencyId>,
  request: JsWord,
  pub parent_module_identifier: Option<ModuleIdentifier>,
  span: Option<ErrorSpan>,
  #[allow(unused)]
  pub ast_path: JsAstPath,
}

impl UrlContextDependency {
  pub fn new(request: JsWord, span: Option<ErrorSpan>, ast_path: JsAstPath) -> Self {
    Self {
      request,
      parent_module_identifier: None,
      span,
      ast_path,
      id: None,
    }
  }
}

impl Dependency for UrlContextDependency {
  fn id(&self) -> Option<DependencyId> {
    self.id
  }
  fn set_id(&mut self, id: Option<DependencyId>) {
    self.id = id;
  }
  fn category(&self) -> &crate::DependencyCategory {
    &crate::DependencyCategory::Url
  }

  fn dependency_type(&self) -> &crate::DependencyType {
    &crate::DependencyType::NewUrlContext
  }

  fn parent_module_identifier(&self) -> Option<&ModuleIdentifier> {
    self.parent_module_identifier.as_ref()
  }

  fn set_parent_module_identifier(&mut self, module_identifier: Option<ModuleIdentifier>) {
    self.parent_module_identifier = module_identifier;
  }
}

impl ModuleDependency for UrlContextDependency {
  fn request(&self) -> &str {
    &self.request
  }

  fn user_request(&self) -> &str {
    &self.request
  }

  fn span(&self) -> Option<&crate::ErrorSpan> {
    None
  }
}

impl CodeGeneratable for UrlContextDependency {
  fn generate(
    &self,
    code_generatable_context: &mut crate::CodeGeneratableContext,
  ) -> Result<CodeGeneratableResult> {
    let CodeGeneratableContext { compilation, .. } = code_generatable_context;
    let mut code_gen = CodeGeneratableResult::default();

    if let Some(id) = self.id() {
      if let Some(module_id) = compilation
        .module_graph
        .module_graph_module_by_dependency_id(&id)
        .map(|m| m.id(&compilation.chunk_graph).to_string())
      {
        code_generatable_context
          .runtime_requirements
          .insert(RuntimeGlobals::BASE_URI);
        code_gen.visitors.push(
          create_javascript_visitor!(exact &self.ast_path, visit_mut_new_expr(n: &mut NewExpr) {
                let Some(args) = &mut n.args else { return };

                if let (Some(first), Some(second)) = (args.first(), args.get(1)) {
                  let path_span = first.span();
                  let meta_span = second.span();

                  let require_call = CallExpr {
                    span: path_span,
                    callee: Callee::Expr(quote_ident!(RuntimeGlobals::REQUIRE).into()),
                    args: vec![ExprOrSpread {
                      spread: None,
                      expr: quote_str!(&*module_id).into(),
                    }],
                    type_args: None,
                  };

                  args[0] = ExprOrSpread {
                    spread: None,
                    expr: require_call.into(),
                  };

                  args[1] = ExprOrSpread {
                    spread: None,
                    expr: member_expr!(meta_span, __webpack_require__.b),
                  };
                }
          }),
        );
      }
    }

    Ok(code_gen)
  }
}
