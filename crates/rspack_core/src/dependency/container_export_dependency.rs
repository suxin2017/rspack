use crate::{
  CodeGeneratable, CodeGeneratableContext, CodeGeneratableResult, Dependency, DependencyCategory,
  DependencyId, DependencyType, ErrorSpan, ModuleDependency,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContainerExportDependency {
  pub(crate) name: String,
  pub(crate) id: Option<DependencyId>,
}

impl CodeGeneratable for ContainerExportDependency {
  fn generate(
    &self,
    _code_generatable_context: &mut CodeGeneratableContext,
  ) -> rspack_error::Result<CodeGeneratableResult> {
    Ok(CodeGeneratableResult::default())
  }
}

impl Dependency for ContainerExportDependency {
  fn category(&self) -> &DependencyCategory {
    &DependencyCategory::Esm
  }
  fn dependency_type(&self) -> &DependencyType {
    &DependencyType::ContainerExposedDependency
  }
  fn id(&self) -> Option<DependencyId> {
    self.id
  }
  fn set_id(&mut self, id: Option<DependencyId>) {
    self.id = id
  }
}

impl ModuleDependency for ContainerExportDependency {
  fn request(&self) -> &str {
    "./export.js"
  }

  fn user_request(&self) -> &str {
    "container export user request"
  }

  fn span(&self) -> Option<&ErrorSpan> {
    None
  }
}
