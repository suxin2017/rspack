use std::any::Any;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use rspack_error::{IntoTWithDiagnosticArray, TWithDiagnosticArray};
use rspack_identifier::{Identifiable, Identifier};
use rspack_sources::{RawSource, Source, SourceExt};
use rspack_util::ext::{DynEq, DynHash};

use crate::DependencyType::ContainerExposedDependency;
use crate::{
  AsModuleDependency, AstOrSource, BuildContext, BuildInfo, BuildMeta, BuildMetaExportsType,
  BuildResult, CodeGenerationResult, Compilation, ContainerEntryDependency,
  ContainerExportDependency, Context, GenerationResult, LibIdentOptions, Module, ModuleFactory,
  ModuleFactoryCreateData, ModuleFactoryResult, ModuleType, RuntimeGlobals, SourceType,
  StaticExportsDependency,
};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct ContainerEntryModule {
  name: String,
  identifier: Identifier,
  // block: Vec<>
}

impl ContainerEntryModule {
  pub fn new(name: String) -> Self {
    Self {
      name,
      identifier: Identifier::from("bac"),
    }
  }
}

impl Identifiable for ContainerEntryModule {
  fn identifier(&self) -> Identifier {
    self.identifier
  }
}

#[async_trait::async_trait]
impl Module for ContainerEntryModule {
  fn module_type(&self) -> &ModuleType {
    &ModuleType::JsDynamic
  }

  fn source_types(&self) -> &[SourceType] {
    &[SourceType::JavaScript]
  }

  fn original_source(&self) -> Option<&dyn Source> {
    None
  }

  fn readable_identifier(&self, _context: &Context) -> Cow<str> {
    Cow::from("contain entry")
  }

  fn size(&self, _source_type: &SourceType) -> f64 {
    42 as f64
  }

  fn code_generation(
    &self,
    compilation: &Compilation,
  ) -> rspack_error::Result<CodeGenerationResult> {
    let mut code_generation_result = CodeGenerationResult::default();
    // RuntimeGlobals.definePropertyGetters,
    // RuntimeGlobals.hasOwnProperty,
    // RuntimeGlobals.exports
    compilation.entry_dependencies.get();
    let mut runtime_globals = RuntimeGlobals::default();
    runtime_globals.add(RuntimeGlobals::DEFINE_PROPERTY_GETTERS);
    runtime_globals.add(RuntimeGlobals::HAS_OWN_PROPERTY);
    runtime_globals.add(RuntimeGlobals::EXPORTS);
    code_generation_result.runtime_requirements = runtime_globals;
    let source = "fff";
    code_generation_result.add(
      SourceType::JavaScript,
      GenerationResult::from(AstOrSource::from(
        RawSource::from("condole.log('ddd');".to_string()).boxed(),
      )),
    );
    Ok(code_generation_result)
  }

  async fn build(
    &mut self,
    build_context: BuildContext<'_>,
  ) -> rspack_error::Result<TWithDiagnosticArray<BuildResult>> {
    let mut buildResult = BuildResult::default();
    buildResult.dependencies = vec![Box::new(ContainerExportDependency {
      name: "container export dependency".to_string(),
      id: None,
    })];

    let mut build_info = BuildInfo::default();
    build_info.strict = true;

    let mut build_meta = BuildMeta::default();
    build_meta.exports_type = BuildMetaExportsType::Namespace;
    Ok(buildResult.with_empty_diagnostic())
  }

  fn lib_ident(&self, _options: LibIdentOptions) -> Option<Cow<str>> {
    Some(Cow::from(format!("webpack/container/entry/{}", self.name)))
  }
}
