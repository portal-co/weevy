use anyhow::Context as AnyhowContext;
use axum::{body::Body, extract::Request};
use base64::Engine;
use swc_atoms::Atom;
use swc_common::{
    FileName, FilePathMapping, SourceMap, Spanned, SyntaxContext,
    errors::{ColorConfig, Handler},
    sync::Lrc,
};
use swc_ecma_ast::{EsVersion, Ident};
use swc_ecma_parser::{Parser, StringInput, Syntax, lexer::Lexer};
use swc_ecma_visit::VisitMutWith;

pub async fn server(a: Request) -> anyhow::Result<Body> {
    let q2 = a.uri().query().context("in getting the query")?;
    let q = q2.split("&");
    let v = match q.clone().find_map(|q| q.strip_prefix("code=")) {
        Some(a) => base64::engine::general_purpose::STANDARD.decode(a)?,
        None => {
            let exter = q
                .clone()
                .find_map(|q| q.strip_prefix("extern="))
                .context("in getting the external url or code")?;
            let exter = base64::engine::general_purpose::STANDARD.decode(exter)?;
            let exter = String::from_utf8(exter)?;
            reqwest::get(exter).await?.bytes().await?.into()
        }
    };
    let v = String::from_utf8(v)?;
    let mut cm = Lrc::new(SourceMap::new(FilePathMapping::default()));
    // let a =
    let fm = cm.new_source_file(FileName::Custom("source.js".into()).into(), v);
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Es(Default::default()),
        // EsVersion defaults to es5
        EsVersion::latest(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let mut prog = parser
        .parse_program()
        .map_err(|mut e| {
            // Unrecoverable fatal error occurred
            e.into_diagnostic(&handler).emit()
        })
        .expect("failed to parser module");

    prog.visit_mut_with(&mut weevy_swc_core::SourceMapper {
        sm: cm.clone(),
        data: Default::default(),
        id: Ident::new(
            Atom::new(format!(
                "__Weevy_{}_Inject",
                base64::engine::general_purpose::STANDARD.encode(q2)
            )),
            prog.span(),
            SyntaxContext::empty(),
        ),
        root: SyntaxContext::empty(),
    });
    prog.visit_mut_with(&mut weevy_swc_core::Wimple {
        root: SyntaxContext::empty(),
        guest_id: Atom::new("MainContent"),
    });

    Ok(Body::new(swc_ecma_codegen::to_code(&prog)))
}
