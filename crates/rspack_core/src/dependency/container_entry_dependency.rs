use crate::{
  CodeGeneratable, CodeGeneratableContext, CodeGeneratableResult, Dependency, DependencyCategory,
  DependencyId, DependencyType, ErrorSpan, ModuleDependency,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContainerEntryDependency {
  pub(crate) name: String,
  pub(crate) id: Option<DependencyId>,
}

impl CodeGeneratable for ContainerEntryDependency {
  fn generate(
    &self,
    _code_generatable_context: &mut CodeGeneratableContext,
  ) -> rspack_error::Result<CodeGeneratableResult> {
    Ok(CodeGeneratableResult::default())
  }
}

impl Dependency for ContainerEntryDependency {
  fn category(&self) -> &DependencyCategory {
    &DependencyCategory::Esm
  }
  fn dependency_type(&self) -> &DependencyType {
    &DependencyType::ContainerEntry
  }
  fn id(&self) -> Option<DependencyId> {
    self.id
  }
  fn set_id(&mut self, id: Option<DependencyId>) {
    self.id = id
  }
}

impl ModuleDependency for ContainerEntryDependency {
  fn request(&self) -> &str {
    "container request"
  }

  fn user_request(&self) -> &str {
    "container user request"
  }

  fn span(&self) -> Option<&ErrorSpan> {
    None
  }
}
