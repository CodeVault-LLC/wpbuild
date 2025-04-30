use swc_common::{FileName, Globals, SourceMap, sync::Lrc};
use swc_ecma_ast::EsVersion;
use swc_ecma_codegen::{self, Config, Emitter, text_writer::JsWriter};
use swc_ecma_parser::{Parser as SwcParser, StringInput, Syntax, TsSyntax, lexer::Lexer};
use swc_ecma_transforms_typescript::strip_type;
use swc_ecma_visit::VisitMutWith;

pub fn transpile_ts(code: &str, filename: String) -> String {
    let cm: Lrc<SourceMap> = SourceMap::default().into();
    let fm = cm.new_source_file(FileName::Custom(filename).into(), code.into());

    let syntax = Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
    });

    let es_version = EsVersion::Es2022;

    let lexer = Lexer::new(syntax, es_version, StringInput::from(&*fm), None);
    let mut parser = SwcParser::new_from(lexer);

    let module = parser.parse_module().expect("Failed to parse TS");

    let globals = Globals::default();
    swc_common::GLOBALS.set(&globals, || {
        let mut module = module.clone();

        // Strip TypeScript types with proper configuration
        module.visit_mut_with(&mut strip_type());

        let mut buf = vec![];
        {
            let mut emitter_config = Config::default();
            emitter_config.minify = true; // Enable minification
            emitter_config.target = es_version;

            let mut emitter = Emitter {
                cfg: emitter_config,
                cm: cm.clone(),
                comments: None,
                wr: Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None)),
            };

            emitter.emit_module(&module).unwrap();
        }
        String::from_utf8(buf).unwrap()
    })
}
