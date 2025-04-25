mod lints;

use lints::Lints;
use syn::{
    Block, Expr, ExprMethodCall, ExprTry, ExprUnsafe, Item, ItemFn, Local, LocalInit, Macro, Pat,
    PatType, Stmt, StmtMacro, Type, TypeReference,
};
use tower_lsp::{
    Client, LanguageServer, LspService, Server,
    jsonrpc::Result,
    lsp_types::{
        Diagnostic, DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams,
        InitializeResult, InitializedParams, PositionEncodingKind, SaveOptions, ServerCapabilities,
        ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
        TextDocumentSyncSaveOptions, Url,
    },
};

const LSP_NAME: &str = env!("CARGO_PKG_NAME");
const LSP_VERSION: &str = env!("CARGO_PKG_VERSION");

struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: LSP_NAME.to_owned(),
                version: Some(LSP_VERSION.to_owned()),
            }),
            capabilities: ServerCapabilities {
                position_encoding: Some(PositionEncodingKind::UTF8),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        #[cfg(debug_assertions)]
        self.client
            .log_message(
                tower_lsp::lsp_types::MessageType::LOG,
                format!(
                    "\nFile {} did open. {:?}",
                    params.text_document.uri.as_str(),
                    params.text_document.uri
                ),
            )
            .await;
        self.parse(params.text_document.uri, params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.parse(
            params.text_document.uri,
            params.content_changes[0].text.clone(),
        )
        .await;
    }

    async fn initialized(&self, _: InitializedParams) {}

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

impl Backend {
    async fn parse(&self, uri: Url, text: String) {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        #[cfg(debug_assertions)]
        let (structure_sender, structure_receiver) = tokio::sync::oneshot::channel();
        tokio::task::spawn_blocking(move || {
            let Ok(file) = syn::parse_file(&text) else {
                let _ = sender.send(Vec::new());
                return;
            };
            let items = file.items.clone();
            drop(file);

            #[cfg(debug_assertions)]
            let _ = structure_sender.send(format!("{items:#?}"));

            let diagnostics = items
                .iter()
                .fold(Vec::new(), |mut accmulator, item| {
                    let Item::Fn(ItemFn { block, .. }) = item else {
                        return accmulator;
                    };

                    let new_diags = Self::check_block(block);

                    accmulator.push(new_diags);
                    accmulator
                })
                .into_iter()
                .flatten()
                .collect();
            let _ = sender.send(diagnostics);
        });

        #[cfg(debug_assertions)]
        self.client
            .log_message(
                tower_lsp::lsp_types::MessageType::LOG,
                structure_receiver.await.unwrap_or_default(),
            )
            .await;

        self.client
            .publish_diagnostics(uri.clone(), receiver.await.unwrap_or_default(), None)
            .await;
    }

    fn check_block(block: &Block) -> Vec<Diagnostic> {
        block.stmts.iter().fold(Vec::new(), |mut accmulator, stmt| {
            accmulator = [
                accmulator,
                Self::check_expresion(stmt),
                Self::check_macro(stmt),
                Self::check_local(stmt),
            ]
            .concat();
            accmulator
        })
    }

    fn check_expresion(stmt: &Stmt) -> Vec<Diagnostic> {
        let Stmt::Expr(expr, _) = stmt else {
            return Vec::new();
        };

        Self::process_expr(expr)
    }

    fn check_macro(stmt: &Stmt) -> Vec<Diagnostic> {
        match stmt {
            Stmt::Macro(StmtMacro { mac, .. }) => Self::process_macro(mac).into_iter().collect(),
            _ => Vec::new(),
        }
    }

    fn check_local(stmt: &Stmt) -> Vec<Diagnostic> {
        match stmt {
            Stmt::Local(Local { pat, init, .. }) => {
                let mut diagnostics = Vec::new();

                if let Pat::Type(PatType { ty, .. }) = pat {
                    if let Type::Reference(TypeReference {
                        lifetime: Some(lifetime),
                        ..
                    }) = ty.as_ref()
                    {
                        if lifetime.ident == "static" {
                            if let Some(diagnostic) = Lints::Static(lifetime.ident.span()).into() {
                                diagnostics.push(diagnostic);
                            }
                        }
                    }
                }

                let Some(LocalInit { expr, .. }) = init else {
                    return diagnostics;
                };

                [diagnostics, Self::process_expr(expr)].concat()
            }
            _ => Vec::new(),
        }
    }

    fn process_expr(expr: &Expr) -> Vec<Diagnostic> {
        match expr {
            Expr::MethodCall(ExprMethodCall { method, .. })
                if method.to_string().as_str() == "unwrap" =>
            {
                <Option<Diagnostic>>::from(Lints::Unwrap(method.span()))
                    .into_iter()
                    .collect()
            }
            Expr::Try(ExprTry { question_token, .. }) => <Option<Diagnostic>>::from(
                Lints::TryExpr(*question_token.spans.first().expect("Should be available.")),
            )
            .into_iter()
            .collect::<Vec<Diagnostic>>(),
            Expr::Unsafe(ExprUnsafe {
                unsafe_token,
                block,
                ..
            }) => {
                let diagnostics = <Option<Diagnostic>>::from(Lints::Unsafe(unsafe_token.span))
                    .into_iter()
                    .collect();
                let sub_block = Self::check_block(block);
                [diagnostics, sub_block].concat()
            }
            _ => Vec::new(),
        }
    }

    fn process_macro(macro_content: &Macro) -> Option<Diagnostic> {
        let segment = macro_content
            .path
            .segments
            .iter()
            .find(|segment| segment.ident.to_string().as_str() == "panic")?;
        Lints::Panic(segment.ident.span()).into()
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
