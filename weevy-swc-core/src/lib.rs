use std::collections::BTreeMap;
use std::mem::take;

use swc_atoms::Atom;
use swc_common::Span;
use swc_common::Spanned;
use swc_common::SyntaxContext;
use swc_common::sync::Lrc;
use swc_ecma_ast::BlockStmt;
use swc_ecma_ast::CallExpr;
use swc_ecma_ast::Callee;
use swc_ecma_ast::ComputedPropName;
use swc_ecma_ast::Decl;
use swc_ecma_ast::Expr;
use swc_ecma_ast::ExprOrSpread;
use swc_ecma_ast::FnExpr;
use swc_ecma_ast::Function;
use swc_ecma_ast::Id;
use swc_ecma_ast::IdentName;
use swc_ecma_ast::Lit;
use swc_ecma_ast::MemberExpr;
use swc_ecma_ast::MemberProp;
use swc_ecma_ast::MethodProp;
use swc_ecma_ast::ModuleDecl;
use swc_ecma_ast::ModuleItem;
use swc_ecma_ast::ObjectLit;
use swc_ecma_ast::Pat;
use swc_ecma_ast::Program;
use swc_ecma_ast::PropOrSpread;
use swc_ecma_ast::ReturnStmt;
use swc_ecma_ast::Script;
use swc_ecma_ast::Stmt;
use swc_ecma_ast::Str;
use swc_ecma_ast::VarDecl;
use swc_ecma_ast::VarDeclarator;
use swc_ecma_ast::{Ident, ImportDecl, Module};
use swc_ecma_visit::VisitMut;
use swc_ecma_visit::VisitMutWith;
pub fn wevy(span: Span, ctx: SyntaxContext) -> Expr {
    Expr::Ident(Ident::new(Atom::new("__WeevyMain"), span, ctx))
}
pub struct SourceMapper {
    pub sm: Lrc<swc_common::SourceMap>,
    pub data: Vec<u8>,
    pub id: Ident,
    pub root: SyntaxContext,
}
impl VisitMut for SourceMapper {
    fn visit_mut_module(&mut self, m: &mut Module) {
        m.visit_mut_children_with(self);
        let s = take(&mut self.data);
        let mut t = vec![];
        brotli::BrotliCompress(&mut &*s, &mut t, &Default::default()).unwrap();
        let s = base2048::encode(&t);
        let s = Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: m.span(),
            ctxt: Default::default(),
            kind: swc_ecma_ast::VarDeclKind::Const,
            declare: true,
            decls: vec![VarDeclarator {
                span: m.span(),
                name: Pat::Ident(self.id.clone().into()),
                definite: false,
                init: Some(Box::new(Expr::Call(CallExpr {
                    span: m.span(),
                    ctxt: Default::default(),
                    callee: swc_ecma_ast::Callee::Expr(Box::new(Expr::Member(MemberExpr {
                        span: m.span(),
                        obj: Box::new(wevy(m.span(), self.root)),
                        prop: swc_ecma_ast::MemberProp::Ident(IdentName {
                            span: m.span(),
                            sym: Atom::new("newSrcDecompressor"),
                        }),
                    }))),
                    args: vec![ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                            span: m.span(),
                            value: Atom::new(s),
                            raw: None,
                        }))),
                    }],
                    type_args: None,
                }))),
            }],
        })));
        m.body = [ModuleItem::Stmt(s)]
            .into_iter()
            .chain(m.body.drain(..))
            .collect();
    }
    fn visit_mut_script(&mut self, m: &mut Script) {
        m.visit_mut_children_with(self);
        let s = take(&mut self.data);
        let mut t = vec![];
        brotli::BrotliCompress(&mut &*s, &mut t, &Default::default()).unwrap();
        let s = base2048::encode(&t);
        let s = Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: m.span(),
            ctxt: Default::default(),
            kind: swc_ecma_ast::VarDeclKind::Const,
            declare: true,
            decls: vec![VarDeclarator {
                span: m.span(),
                name: Pat::Ident(self.id.clone().into()),
                definite: false,
                init: Some(Box::new(Expr::Call(CallExpr {
                    span: m.span(),
                    ctxt: Default::default(),
                    callee: swc_ecma_ast::Callee::Expr(Box::new(Expr::Member(MemberExpr {
                        span: m.span(),
                        obj: Box::new(wevy(m.span(), self.root)),
                        prop: swc_ecma_ast::MemberProp::Ident(IdentName {
                            span: m.span(),
                            sym: Atom::new("newSrcDecompressor"),
                        }),
                    }))),
                    args: vec![ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                            span: m.span(),
                            value: Atom::new(s),
                            raw: None,
                        }))),
                    }],
                    type_args: None,
                }))),
            }],
        })));
        m.body = [s].into_iter().chain(m.body.drain(..)).collect();
    }
    fn visit_mut_decl(&mut self, node: &mut Decl) {
        if let Decl::Fn(f) = node.clone() {
            *node = Decl::Var(Box::new(VarDecl {
                span: f.span(),
                ctxt: f.ident.ctxt.clone(),
                kind: swc_ecma_ast::VarDeclKind::Const,
                declare: true,
                decls: vec![VarDeclarator {
                    span: f.span(),
                    name: f.ident.clone().into(),
                    init: Some(Box::new(Expr::Fn(FnExpr {
                        ident: Some(f.ident),
                        function: f.function,
                    }))),
                    definite: false,
                }],
            }))
        }
        node.visit_mut_children_with(self);
    }
    fn visit_mut_expr(&mut self, node: &mut Expr) {
        node.visit_mut_children_with(self);
        if let Expr::Fn(f) = node.clone() {
            let l = self.data.len();
            self.data.extend(self.sm.span_to_string(f.span()).bytes());
            let m = self.data.len();
            *node = Expr::Call(CallExpr {
                span: f.span(),
                ctxt: SyntaxContext::empty(),
                callee: swc_ecma_ast::Callee::Expr(Box::new(Expr::Ident(self.id.clone()))),
                args: vec![
                    ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                            span: f.span(),
                            raw: None,
                            value: Atom::new(format!("{l};{m}")),
                        }))),
                    },
                    ExprOrSpread {
                        spread: None,
                        expr: Box::new(take(node)),
                    },
                ],
                type_args: None,
            })
        }
        if let Expr::Arrow(f) = node.clone() {
            let l = self.data.len();
            self.data.extend(self.sm.span_to_string(f.span()).bytes());
            let m = self.data.len();
            *node = Expr::Call(CallExpr {
                span: f.span(),
                ctxt: SyntaxContext::empty(),
                callee: swc_ecma_ast::Callee::Expr(Box::new(Expr::Ident(self.id.clone()))),
                args: vec![
                    ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                            span: f.span(),
                            raw: None,
                            value: Atom::new(format!("{l};{m}")),
                        }))),
                    },
                    ExprOrSpread {
                        spread: None,
                        expr: Box::new(take(node)),
                    },
                ],
                type_args: None,
            })
        }
    }
}
pub struct Wimple {
    pub root: SyntaxContext,
    pub guest_id: Atom,
}
impl VisitMut for Wimple {
    fn visit_mut_with_stmt(&mut self, node: &mut swc_ecma_ast::WithStmt) {
        node.visit_mut_children_with(self);
        node.obj = Box::new(Expr::Call(CallExpr {
            span: node.span,
            ctxt: self.root,
            callee: swc_ecma_ast::Callee::Expr(Box::new(Expr::Member(MemberExpr {
                span: node.span,
                obj: Box::new(wevy(node.span, self.root)),
                prop: swc_ecma_ast::MemberProp::Ident(IdentName {
                    span: node.span,
                    sym: Atom::new("withProxy"),
                }),
            }))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: take(&mut node.obj),
            }],
            type_args: None,
        }))
    }
    fn visit_mut_expr(&mut self, e: &mut Expr) {
        'a: {
            if let Expr::Call(c) = e {
                if let Callee::Expr(e) = &mut c.callee {
                    if let Expr::Ident(i) = &**e {
                        if i.sym.as_str() == "eval" {
                            c.args.visit_mut_children_with(self);
                            if let Some(e) = c.args.get_mut(0) {
                                let e = &mut *e.expr;
                                let w = wevy(e.span(), self.root);
                                let w =
                                    [Atom::new("guests"), self.guest_id.clone(), Atom::new("rewrite")]
                                        .into_iter()
                                        .fold(w, |e, a| {
                                            let s = e.span();
                                            Expr::Member(MemberExpr {
                                                span: e.span(),
                                                obj: Box::new(e),
                                                prop: swc_ecma_ast::MemberProp::Ident(IdentName {
                                                    span: s,
                                                    sym: a,
                                                }),
                                            })
                                        });
                                let e2 = take(e);
                                *e = Expr::Call(CallExpr {
                                    span: e2.span(),
                                    ctxt: self.root,
                                    callee: Callee::Expr(Box::new(w)),
                                    args: vec![ExprOrSpread {
                                        spread: None,
                                        expr: Box::new(e2),
                                    }],
                                    type_args: None,
                                });
                            }
                            break 'a;
                        }
                    }
                    if let Expr::Member(m) = &mut **e {
                        if let MemberProp::Computed(_) | MemberProp::Ident(_) = &m.prop {
                            c.args.visit_mut_children_with(self);
                            m.visit_mut_children_with(self);
                            if *m.obj.as_ref() == wevy(m.obj.span(), self.root) {
                                return;
                            }
                            // let e = &mut **e;
                            let w = wevy(m.span(), self.root);
                            let w = [
                                Atom::new("guests"),
                                self.guest_id.clone(),
                                Atom::new("ofThis"),
                            ]
                            .into_iter()
                            .fold(w, |e, a| {
                                let s = e.span();
                                Expr::Member(MemberExpr {
                                    span: e.span(),
                                    obj: Box::new(e),
                                    prop: swc_ecma_ast::MemberProp::Ident(IdentName {
                                        span: s,
                                        sym: a,
                                    }),
                                })
                            });
                            let m = take(m);
                            // let e2 = take(&mut **e);
                            **e = Expr::Call(CallExpr {
                                span: m.span(),
                                ctxt: self.root,
                                callee: Callee::Expr(Box::new(w)),
                                args: vec![
                                    ExprOrSpread {
                                        spread: None,
                                        expr: m.obj,
                                    },
                                    ExprOrSpread {
                                        spread: None,
                                        expr: match m.prop {
                                            MemberProp::Ident(ident_name) => {
                                                Box::new(Expr::Lit(Lit::Str(Str {
                                                    span: ident_name.span,
                                                    value: ident_name.sym,
                                                    raw: None,
                                                })))
                                            }
                                            MemberProp::PrivateName(private_name) => todo!(),
                                            MemberProp::Computed(computed_prop_name) => {
                                                computed_prop_name.expr
                                            }
                                        },
                                    },
                                ],
                                type_args: None,
                            });
                            break 'a;
                        }
                    }
                }
            }
            e.visit_mut_children_with(self);
        }
        // };
        if let Expr::Lit(_) | Expr::Cond(_) | Expr::Unary(_) | Expr::Bin(_) = e {
            return;
        }
        let w = wevy(e.span(), self.root);
        let w = [Atom::new("guests"), self.guest_id.clone(), Atom::new("of")]
            .into_iter()
            .fold(w, |e, a| {
                let s = e.span();
                Expr::Member(MemberExpr {
                    span: e.span(),
                    obj: Box::new(e),
                    prop: swc_ecma_ast::MemberProp::Ident(IdentName { span: s, sym: a }),
                })
            });
        let e2 = take(e);
        *e = Expr::Call(CallExpr {
            span: e2.span(),
            ctxt: self.root,
            callee: Callee::Expr(Box::new(w)),
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(e2),
            }],
            type_args: None,
        });
    }
}
