use std::fmt::{Debug, Formatter};

use crate::dependency::ContainerEntryDependency;
use crate::{
  ApplyContext, Compilation, CompilationArgs, EntryItem, Plugin, PluginCompilationHookOutput,
  PluginContext, PluginMakeHookOutput, PluginThisCompilationHookOutput, ThisCompilationArgs,
};

#[derive(Debug, Default)]
pub struct ContainerPlugin {
  options: ContainerPluginOptions,
}

#[derive(Debug, Default)]
pub struct ContainerPluginOptions {
  /**
   * Modules that should be exposed by this container. When provided, property name is used as public name, otherwise public name is automatically inferred from request.
   */
  pub exposes: String,
  /**
   * The filename for this container relative path inside the `output.path` directory.
   */
  pub filename: Option<String>,
  /**
   * Options for library.
   */
  // library: LibraryOptions,
  /**
   * The name for this container.
   */
  pub name: String,
  /**
   * The name of the runtime chunk. If set a runtime chunk with this name is created or an existing entrypoint is used as runtime.
   */
  // runtime?: EntryRuntime;
  /**
   * The name of the share scope which is shared with the host (defaults to 'default').
   */
  pub share_scope: Option<String>,
}

impl ContainerPlugin {
  pub fn new(options: ContainerPluginOptions) -> ContainerPlugin {
    Self { options }
  }
}

#[async_trait::async_trait]
impl Plugin for ContainerPlugin {
  fn apply(&mut self, _ctx: PluginContext<&mut ApplyContext>) -> rspack_error::Result<()> {
    let options = &self.options;
    Ok(())
  }
  async fn compilation(&mut self, args: CompilationArgs<'_>) -> PluginCompilationHookOutput {
    args.compilation.add_entry(
      "a".to_string(),
      Box::new(ContainerEntryDependency {
        name: "b".to_string(),
        id: None,
      }),
      EntryItem {
        import: vec![],
        runtime: None,
      },
    );

    Ok(())
  }
}
