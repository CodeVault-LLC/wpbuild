use std::{collections::HashMap, path::Path};

use anyhow::{Error, Result};
use swc_bundler::{Bundler, Config, Hook, Load, ModuleData, ModuleRecord, ModuleType, Resolve};
use swc_common::{sync::Lrc, FileName, FilePathMapping, Globals, SourceMap, Span};
use swc_ecma_ast::{KeyValueProp, PropName};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_loader::resolve::Resolution;
use swc_ecma_parser::{parse_file_as_module, Syntax};

pub fn bundle_entry(entry_file: &str) -> Result<String> {
    let path = Path::new(entry_file);

    let mut entries: HashMap<String, FileName> = HashMap::new();
    entries.insert(path.to_string_lossy().to_string(), FileName::Real(path.into()));

    let globals: Globals = Globals::new();
    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::new(FilePathMapping::empty()));
    let external_modules = Vec::new();

    let mut bundler = Bundler::new(
        &globals,
        cm.clone(),
        PathLoader { cm: cm.clone() },
        PathResolver,
        Config {
            require: true,
            module: ModuleType::Es,
            external_modules,
            ..Default::default()
        },
        Box::new(Noop),
    );

    let mut bundles = bundler.bundle(entries).expect("failed to bundle");
    let bundle = bundles.pop().unwrap();

    let mut buf = Vec::new(); // Capture into a memory buffer
    let mut emitter = Emitter {
        cfg: swc_ecma_codegen::Config::default(),
        cm: cm.clone(),
        comments: None,
        wr: Box::new(JsWriter::new(cm, "\n", &mut buf, None)),
    };

    emitter.emit_module(&bundle.module).expect("failed to emit module");

    let result = String::from_utf8(buf).expect("emitted code is not valid UTF-8");
    Ok(result)
}



struct PathLoader {
    cm: Lrc<SourceMap>,
}

impl Load for PathLoader {
    fn load(&self, file: &FileName) -> Result<ModuleData, Error> {
        let file = match file {
            FileName::Real(v) => v,
            _ => unreachable!(),
        };

        println!("Loading file: {}", file.display());

        let fm = self.cm.load_file(file)?;

        let module = parse_file_as_module(
            &fm,
            Syntax::Es(Default::default()),
            Default::default(),
            None,
            &mut Vec::new(),
        )
        .expect("This should not happen");

        Ok(ModuleData {
            fm,
            module,
            helpers: Default::default(),
        })
    }
}
struct PathResolver;

impl Resolve for PathResolver {
    fn resolve(&self, base: &FileName, module_specifier: &str) -> Result<Resolution, Error> {
        assert!(
            module_specifier.starts_with('.'),
            "We are not using node_modules within this example"
        );

        let base = match base {
            FileName::Real(v) => v,
            _ => unreachable!(),
        };

        Ok(Resolution {
            filename: FileName::Real(
                base.parent()
                    .unwrap()
                    .join(module_specifier)
                    .with_extension("js"),
            ),
            slug: None,
        })
    }
}

struct Noop;

impl Hook for Noop {
    fn get_import_meta_props(&self, _: Span, _: &ModuleRecord) -> Result<Vec<KeyValueProp>, Error> {
        let temp_value = KeyValueProp {
            key: PropName::Str(("test".into())),
            value: Box::new(swc_ecma_ast::Expr::Lit(swc_ecma_ast::Lit::Str(
                swc_ecma_ast::Str {
                    span: Default::default(),
                    value: "test".into(),
                    raw: None,
                },
            ))),
        };
        
        Ok(vec![temp_value])
    }
}