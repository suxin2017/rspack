use rspack_error::{IntoTWithDiagnosticArray, TWithDiagnosticArray};

use crate::container_entry_module::ContainerEntryModule;
use crate::{
  AsModuleDependency, BoxModule, ModuleFactory, ModuleFactoryCreateData, ModuleFactoryResult,
};

pub struct ContainerEntryModuleFactory {}

#[async_trait::async_trait]
impl ModuleFactory for ContainerEntryModuleFactory {
  async fn create(
    self,
    data: ModuleFactoryCreateData,
  ) -> rspack_error::Result<TWithDiagnosticArray<ModuleFactoryResult>> {
    let dependency = data.dependency;
    dbg!("create");

    Ok(
      ModuleFactoryResult::new(Box::new(ContainerEntryModule::new("abc".to_string())))
        .with_empty_diagnostic(),
    )
  }
}
