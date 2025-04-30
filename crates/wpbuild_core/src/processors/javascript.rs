use swc_common::{FileName, Globals, SourceMap, sync::Lrc};
use swc_ecma_ast::EsVersion;
use swc_ecma_codegen::{self, Config, Emitter, text_writer::JsWriter};
use swc_ecma_parser::{EsSyntax, Parser as SwcParser, StringInput, Syntax, lexer::Lexer};

pub fn transpile_js(code: &str, filename: String) -> String {
    let cm: Lrc<SourceMap> = SourceMap::default().into();
    let fm: Lrc<swc_common::SourceFile> =
        cm.new_source_file(FileName::Custom(filename).into(), code.into());

    let syntax: Syntax = Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    });

    let es_version: EsVersion = EsVersion::Es2022;

    let lexer: Lexer<'_> = Lexer::new(syntax, es_version, StringInput::from(&*fm), None);
    let mut parser: SwcParser<Lexer<'_>> = SwcParser::new_from(lexer);

    let module: swc_ecma_ast::Module = parser.parse_module().expect("Failed to parse JS");

    let mut emitter_config: Config = Config::default();
    emitter_config.target = EsVersion::Es2022;
    emitter_config.minify = true; // Enable minification for JS files too

    let globals: Globals = Globals::default();
    swc_common::GLOBALS.set(&globals, || {
        let mut buf: Vec<u8> = vec![];
        {
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
