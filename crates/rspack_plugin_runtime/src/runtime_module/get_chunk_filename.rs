use std::cmp::Ordering;

use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use rspack_core::{
  get_css_chunk_filename_template, get_filename_without_hash_length,
  get_js_chunk_filename_template, impl_runtime_module,
  rspack_sources::{BoxSource, RawSource, SourceExt},
  ChunkUkey, Compilation, Filename, PathData, RuntimeGlobals, RuntimeModule, SourceType,
};
use rspack_identifier::Identifier;

use super::create_fake_chunk;
use super::stringify_dynamic_chunk_map;
use super::stringify_static_chunk_map;
use super::utils::chunk_has_css;
use crate::runtime_module::unquoted_stringify;

#[derive(Debug, Eq)]
pub struct GetChunkFilenameRuntimeModule {
  id: Identifier,
  chunk: Option<ChunkUkey>,
  content_type: &'static str,
  source_type: SourceType,
  global: RuntimeGlobals,
  all_chunks: bool,
}
// It's render is different with webpack, rspack will only render chunk map<chunkId, chunkName>
// and search it.
impl GetChunkFilenameRuntimeModule {
  pub fn new(
    content_type: &'static str,
    source_type: SourceType,
    global: RuntimeGlobals,
    all_chunks: bool,
  ) -> Self {
    Self {
      id: Identifier::from(format!("webpack/runtime/get_chunk_filename/{content_type}")),
      chunk: None,
      content_type,
      source_type,
      global,
      all_chunks,
    }
  }
}

impl RuntimeModule for GetChunkFilenameRuntimeModule {
  fn name(&self) -> Identifier {
    self.id
  }

  fn cacheable(&self) -> bool {
    false
  }

  fn generate(&self, compilation: &Compilation) -> BoxSource {
    let chunks = self
      .chunk
      .and_then(|chunk_ukey| compilation.chunk_by_ukey.get(&chunk_ukey))
      .map(|chunk| {
        if self.all_chunks {
          chunk.get_all_referenced_chunks(&compilation.chunk_group_by_ukey)
        } else {
          let mut chunks = chunk.get_all_async_chunks(&compilation.chunk_group_by_ukey);
          if compilation
            .chunk_graph
            .get_tree_runtime_requirements(&chunk.ukey)
            .contains(RuntimeGlobals::ENSURE_CHUNK_INCLUDE_ENTRIES)
          {
            chunks.extend(
              compilation
                .chunk_graph
                .get_chunk_entry_dependent_chunks_iterable(
                  &chunk.ukey,
                  &compilation.chunk_by_ukey,
                  &compilation.chunk_group_by_ukey,
                ),
            );
          }
          for entrypoint in
            chunk.get_all_referenced_async_entrypoints(&compilation.chunk_group_by_ukey)
          {
            let entrypoint = compilation.chunk_group_by_ukey.expect_get(&entrypoint);
            chunks.insert(entrypoint.get_entry_point_chunk());
          }
          chunks
        }
      });

    let mut dynamic_filename: Option<&Filename> = None;
    let mut max_chunk_set_size = 0;
    let mut chunk_filenames = IndexMap::new();
    let mut chunk_map = IndexMap::new();

    if let Some(chunks) = chunks {
      chunks
        .iter()
        .filter_map(|chunk_ukey| compilation.chunk_by_ukey.get(chunk_ukey))
        .for_each(|chunk| {
          let filename_template = match self.source_type {
            // TODO webpack different
            // css chunk will generate a js chunk, so here add it.
            SourceType::JavaScript => Some(get_js_chunk_filename_template(
              chunk,
              &compilation.options.output,
              &compilation.chunk_group_by_ukey,
            )),
            SourceType::Css => chunk_has_css(&chunk.ukey, compilation).then(|| {
              get_css_chunk_filename_template(
                chunk,
                &compilation.options.output,
                &compilation.chunk_group_by_ukey,
              )
            }),
            _ => unreachable!(),
          };

          if let Some(filename_template) = filename_template {
            chunk_map.insert(&chunk.ukey, chunk);

            let chunk_set = chunk_filenames
              .entry(filename_template.clone())
              .or_insert(IndexSet::new());

            chunk_set.insert(&chunk.ukey);

            let should_update = match dynamic_filename {
              Some(dynamic_filename) => match chunk_set.len().cmp(&max_chunk_set_size) {
                Ordering::Less => false,
                Ordering::Greater => true,
                Ordering::Equal => match filename_template
                  .template()
                  .len()
                  .cmp(&dynamic_filename.template().len())
                {
                  Ordering::Less => false,
                  Ordering::Greater => true,
                  Ordering::Equal => !matches!(
                    filename_template
                      .template()
                      .cmp(dynamic_filename.template()),
                    Ordering::Less,
                  ),
                },
              },
              None => true,
            };
            if should_update {
              max_chunk_set_size = chunk_set.len();
              dynamic_filename = Some(filename_template);
            }
          }
        });
    }

    let dynamic_url = dynamic_filename
      .and_then(|filename| {
        chunk_filenames
          .get(filename)
          .map(|chunks| (filename, chunks))
      })
      .map(|(filename, chunks)| {
        let (fake_filename, hash_len_map) = get_filename_without_hash_length(filename);
        let fake_chunk = create_fake_chunk(
          Some("\" + chunkId + \"".to_string()),
          Some(stringify_dynamic_chunk_map(
            |c| match &c.name {
              Some(name) => Some(name.to_string()),
              None => c.id.clone().map(|id| id.to_string()),
            },
            chunks,
            &chunk_map,
          )),
          Some(stringify_dynamic_chunk_map(
            |c| {
              let hash = c.rendered_hash.as_ref().map(|hash| hash.to_string());
              match hash_len_map.get("[chunkhash]") {
                Some(hash_len) => hash.map(|s| s[..*hash_len].to_string()),
                None => hash,
              }
            },
            chunks,
            &chunk_map,
          )),
        );

        let content_hash = Some(stringify_dynamic_chunk_map(
          |c| {
            c.content_hash.get(&self.source_type).map(|i| {
              let hash = i
                .rendered(compilation.options.output.hash_digest_length)
                .to_string();
              match hash_len_map.get("[contenthash]") {
                Some(hash_len) => hash[..*hash_len].to_string(),
                None => hash,
              }
            })
          },
          chunks,
          &chunk_map,
        ));

        let full_hash = match hash_len_map
          .get("[fullhash]")
          .or(hash_len_map.get("[hash]"))
        {
          Some(hash_len) => format!(
            "\" + {}().slice(0, {}) + \"",
            RuntimeGlobals::GET_FULL_HASH,
            hash_len
          ),
          None => format!("\" + {}() + \"", RuntimeGlobals::GET_FULL_HASH),
        };

        format!(
          "\"{}\"",
          compilation.get_path(
            &fake_filename,
            PathData::default()
              .chunk(&fake_chunk)
              .hash_optional(Some(full_hash.as_str()))
              .content_hash_optional(content_hash.as_deref()),
          )
        )
      });

    let mut static_urls = IndexMap::new();
    for (filename_template, chunks) in
      chunk_filenames
        .iter()
        .filter(|(filename, _)| match dynamic_filename {
          None => true,
          Some(dynamic_filename) => dynamic_filename != *filename,
        })
    {
      for chunk_ukey in chunks.iter() {
        if let Some(chunk) = chunk_map.get(chunk_ukey) {
          let (fake_filename, hash_len_map) = get_filename_without_hash_length(filename_template);

          let fake_chunk = create_fake_chunk(
            chunk
              .id
              .as_ref()
              .map(|chunk_id| unquoted_stringify(chunk, chunk_id)),
            match &chunk.name {
              Some(chunk_name) => Some(unquoted_stringify(chunk, chunk_name)),
              None => chunk
                .id
                .as_ref()
                .map(|chunk_id| unquoted_stringify(chunk, chunk_id)),
            },
            chunk.rendered_hash.as_ref().map(|chunk_hash| {
              let hash = unquoted_stringify(chunk, &chunk_hash.as_ref().to_string());
              match hash_len_map.get("[chunkhash]") {
                Some(hash_len) => hash[..*hash_len].to_string(),
                None => hash,
              }
            }),
          );

          let content_hash = chunk.content_hash.get(&self.source_type).map(|i| {
            let hash = unquoted_stringify(
              chunk,
              &i.rendered(compilation.options.output.hash_digest_length)
                .to_string(),
            );
            match hash_len_map.get("[contenthash]") {
              Some(hash_len) => hash[..*hash_len].to_string(),
              None => hash,
            }
          });

          let full_hash = match hash_len_map
            .get("[fullhash]")
            .or(hash_len_map.get("[hash]"))
          {
            Some(hash_len) => format!(
              "\" + {}().slice(0, {}) + \"",
              RuntimeGlobals::GET_FULL_HASH,
              hash_len
            ),
            None => format!("\" + {}() + \"", RuntimeGlobals::GET_FULL_HASH),
          };

          let filename = format!(
            "\"{}\"",
            compilation.get_path(
              &fake_filename,
              PathData::default()
                .chunk(&fake_chunk)
                .hash_optional(Some(full_hash.as_str()))
                .content_hash_optional(content_hash.as_deref())
            ),
          );

          if let Some(chunk_id) = &chunk.id {
            static_urls
              .entry(filename)
              .or_insert(Vec::new())
              .push(chunk_id);
          }
        }
      }
    }

    RawSource::from(format!(
      "// This function allow to reference chunks
        {} = function (chunkId) {{
          // return url for filenames not based on template
          {}
          // return url for filenames based on template
          return {};
        }};
      ",
      self.global,
      static_urls
        .iter()
        .map(|(filename, chunk_ids)| stringify_static_chunk_map(filename, chunk_ids))
        .join("\n"),
      dynamic_url.unwrap_or_else(|| format!("\"\" + chunkId + \".{}\"", self.content_type))
    ))
    .boxed()
  }

  fn attach(&mut self, chunk: ChunkUkey) {
    self.chunk = Some(chunk);
  }
}

impl_runtime_module!(GetChunkFilenameRuntimeModule);
