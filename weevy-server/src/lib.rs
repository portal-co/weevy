use anyhow::Context as AnyhowContext;
use axum::{body::Body, extract::Request};
use base64::Engine;
use sha3::Digest;
use swc_atoms::Atom;
use swc_common::{
    FileName, FilePathMapping, SourceMap, Spanned, SyntaxContext,
    errors::{ColorConfig, Handler},
    sync::Lrc,
};
use swc_ecma_ast::{EsVersion, Expr, Ident, IdentName, MemberExpr, Program};
use swc_ecma_parser::{Parser, StringInput, Syntax, lexer::Lexer};
use swc_ecma_visit::VisitMutWith;
use weevy_swc_core::{default_source_mapper, wevy};
pub trait Plugin {
    type State;
    async fn state(&self, a: &Request, client: &reqwest::Client) -> anyhow::Result<Self::State>;
    type RewriteState;
    async fn rewrite_state(
        &self,
        s: Self::State,
        body: &[u8],
        ty: &str,
    ) -> anyhow::Result<Self::RewriteState>;
    async fn rewrite_js(
        &self,
        state: &mut Self::RewriteState,
        prog: &mut Program,
    ) -> anyhow::Result<()>;
}
pub async fn server(
    a: Request,
    plugin: &impl Plugin,
    client: &reqwest::Client,
) -> anyhow::Result<Body> {
    let mut state = plugin
        .state(&a, client)
        .await
        .context("in preparing the plugin")?;
    let q2 = a.uri().query().context("in getting the query")?;
    let q = q2.split("&");
    let gid = q
        .clone()
        .find_map(|q| q.strip_prefix("gid="))
        .unwrap_or("MainContent");
    let (v, ct) = match q.clone().find_map(|q| q.strip_prefix("code=")) {
        Some(a) => (
            base64::engine::general_purpose::STANDARD.decode(a)?,
            q.clone()
                .find_map(|q| q.strip_prefix("ct="))
                .context("in getting the content type")?
                .to_owned(),
        ),
        None => {
            let exter = q
                .clone()
                .find_map(|q| q.strip_prefix("extern="))
                .context("in getting the external url or code")?;
            let exter = base64::engine::general_purpose::STANDARD.decode(exter)?;
            let exter = String::from_utf8(exter)?;
            let q = client.execute(client.get(exter).build()?).await?;
            let Some(ct) = q
                .headers()
                .get("Content-Type")
                .and_then(|a| a.to_str().ok())
            else {
                return Ok(q.bytes().await?.into());
            };
            let ct = ct.to_owned();
            (q.bytes().await?.into(), ct)
        }
    };
    let mut state = plugin
        .rewrite_state(state, &v, ct.as_str())
        .await
        .context("in preparing the plugin for rewriting")?;
    let mut cm = Lrc::new(SourceMap::new(FilePathMapping::default()));
    match ct.as_str() {
        "text/javascript" | "application/javascript" => {
            let v = String::from_utf8(v)?;

            // let a =
            let fm = cm.new_source_file(FileName::Custom("source.js".into()).into(), v);
            let handler =
                Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
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
                .map_err(|e| e.into_diagnostic(&handler))
                .map_err(|e| anyhow::anyhow!("core: {e:?}"))?;

            prog.visit_mut_with(&mut weevy_swc_core::SourceMapper {
                sm: cm.clone(),
                data: Default::default(),
                id: Ident::new(
                    Atom::new(format!(
                        "__Weevy_{}_Inject",
                        base64::engine::general_purpose::STANDARD
                            .encode(sha3::Sha3_256::digest(q2))
                    )),
                    prog.span(),
                    SyntaxContext::empty(),
                ),
                root: SyntaxContext::empty(),
                source_mapper: default_source_mapper(&prog, SyntaxContext::empty()),
            });
            prog.visit_mut_with(&mut weevy_swc_core::Wimple {
                root: SyntaxContext::empty(),
                guest_id: Atom::new(gid),
            });
            plugin
                .rewrite_js(&mut state, &mut prog)
                .await
                .context("in rewriting using the plugin")?;

            Ok(Body::new(swc_ecma_codegen::to_code(&prog)))
        }
        _ => Ok(v.into()),
    }
}
