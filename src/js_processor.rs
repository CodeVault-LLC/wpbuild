use swc_common::{FileName, Globals, SourceMap, sync::Lrc};
use swc_ecma_codegen::{self, Emitter, text_writer::JsWriter, Config};
use swc_ecma_parser::{lexer::Lexer, EsSyntax, Parser as SwcParser, StringInput, Syntax, TsSyntax};
use swc_ecma_ast::EsVersion;
use swc_ecma_transforms_typescript::strip_type;
use swc_ecma_visit::VisitMutWith;

