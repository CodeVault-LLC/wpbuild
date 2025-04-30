use swc_common::{FileName, Globals, SourceMap};
use swc_ecma_codegen::{self, Emitter, text_writer::JsWriter, Config};
use swc_ecma_parser::{lexer::Lexer, EsSyntax, Parser as SwcParser, StringInput, Syntax};
use swc_ecma_ast::EsVersion;
use swc_ecma_transforms::fixer;
use swc_ecma_transforms_optimization::simplify::dce;
use swc_ecma_transforms_base::resolver;
use swc_ecma_visit::FoldWith;

pub fn transpile_js(code: &str, filename: String) -> String {
    let cm = SourceMap::default();
    let fm = cm.new_source_file(FileName::Custom(filename).into(), code.into());

    let syntax = Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    });

    let es_version = EsVersion::Es2022;

    let lexer = Lexer::new(syntax, es_version, StringInput::from(&*fm), None);
    let mut parser = SwcParser::new_from(lexer);

    let mut module = parser.parse_module().expect("Failed to parse JS");

    let emitter_config = Config {
        target: EsVersion::Es2015
        minify: false,
        ..Default::default(),
    };

    let globals = Globals::default();
    swc_common::GLOBALS.set(&globals, || {
        let mut buf = vec![];
        {
            let mut emitter = Emitter {
                cfg: emitter_config,
                cm: cm.clone(),
                comments: None,
                wr: Box::new(JsWriter::new(
                    cm.clone(),
                    "\n",
                    &mut buf,
                    None,
                )),
            };

            emitter.emit_module(&module).unwrap();
        }

        String::from_utf8(buf).unwrap()
    })
}
