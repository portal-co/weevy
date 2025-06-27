use swc_atoms::Atom;
use swc_common::{FilePathMapping, Mark, SourceMap, Span, Spanned, SyntaxContext, errors::Handler};
use swc_ecma_ast::{Expr, Ident, IdentName, MemberExpr};
use swc_ecma_parser::{Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::VisitMutWith;
use wasm_bindgen::prelude::*;
use weevy_camo::Config;
use weevy_swc_core::{distanced_source_mapper, single_tenant_source_mapper, wevy, ApplyCamo, SourceMapper};

#[wasm_bindgen]
pub struct Ctx {
    camo: Config<String>,
    sm: swc_common::sync::Lrc<SourceMap>,
}
impl Ctx {
    fn _rewrite(&self, a: &str) -> String {
        let fm = self.sm.new_source_file(
            swc_common::sync::Lrc::new(swc_common::FileName::Anon),
            a.to_owned(),
        );
        let handler =
            Handler::with_emitter_writer(Box::new(std::io::sink()), Some(self.sm.clone()));

        // Real usage
        // let fm = cm
        //     .load_file(Path::new("test.js"))
        //     .expect("failed to load test.js");
        // let fm = cm.new_source_file(
        //     FileName::Custom("test.js".into()).into(),
        //     "function foo() {}".into(),
        // );
        let lexer = Lexer::new(
            // We want to parse ecmascript
            Syntax::Es(Default::default()),
            // EsVersion defaults to es5
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        for e in parser.take_errors() {
            return a.to_owned();
        }

        let Ok(mut module) = parser.parse_program() else {
            return a.to_owned();
        };
        module.visit_mut_with(&mut ApplyCamo {
            cfg: Config {
                isolate: self.camo.isolate.as_str(),
                flags: self.camo.flags,
            },
            applier: match distanced_source_mapper(
                &Span::dummy_with_cmt(),
                Atom::new("__WeevyMain"),
                SyntaxContext::default(),
            ) {
                w => ["camo", "rewrite"]
                    .into_iter()
                    .fold(w, |e, s| match e.span() {
                        span => Expr::Member(MemberExpr {
                            span: e.span(),
                            obj: Box::new(e),
                            prop: swc_ecma_ast::MemberProp::Ident(IdentName {
                                span,
                                sym: Atom::new(s),
                            }),
                        }),
                    }),
            },
            core: Some((
                Atom::new("camoObj"),
                SyntaxContext::empty().apply_mark(Mark::new()),
            )),
        });
        module.visit_mut_with(&mut SourceMapper::new(
            Default::default(),
            Ident::new(
                Atom::new("__WeevyContainer"),
                Span::dummy_with_cmt(),
                SyntaxContext::empty().apply_mark(Mark::new()),
            ),
            self.sm.clone(),
            match distanced_source_mapper(
                &Span::dummy_with_cmt(),
                Atom::new("__WeevyMain"),
                SyntaxContext::default(),
            ) {
                w => ["newSrcDecompressor"]
                    .into_iter()
                    .fold(w, |e, s| match e.span() {
                        span => Expr::Member(MemberExpr {
                            span: e.span(),
                            obj: Box::new(e),
                            prop: swc_ecma_ast::MemberProp::Ident(IdentName {
                                span,
                                sym: Atom::new(s),
                            }),
                        }),
                    }),
            },
        ));

        return swc_ecma_codegen::to_code(&module);
    }
}
#[wasm_bindgen]
impl Ctx {
    #[wasm_bindgen(constructor)]
    pub fn new(a: &str) -> Self {
        Self {
            camo: Config {
                isolate: a.to_owned(),
                flags: Default::default(),
            },
            sm: swc_common::sync::Lrc::new(SourceMap::new(FilePathMapping::empty())),
        }
    }
    #[wasm_bindgen]
    pub fn rewrite(&self, a: &str) -> String {
        self._rewrite(a)
    }
}
