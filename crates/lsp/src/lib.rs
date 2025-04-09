use anyhow::Result;
use chumsky::error::Rich;
use log::info;
use lsp_server::{Connection, Message, Notification, Request, Response};
use lsp_types::{
    Diagnostic, DiagnosticSeverity, Position, PublishDiagnosticsParams, Range, ServerCapabilities,
    TextDocumentContentChangeEvent,
};
use phprs_lexer::Token;

pub async fn run() -> Result<()> {
    info!("Starting LSP server...");

    let (connection, io_threads) = Connection::stdio();

    let capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
            lsp_types::TextDocumentSyncKind::FULL,
        )),
        completion_provider: None,
        definition_provider: None,
        ..ServerCapabilities::default()
    })
    .unwrap();

    let initialize_params = match connection.initialize(capabilities) {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }

            return Err(e.into());
        }
    };

    run_server(connection, initialize_params).await?;

    Ok(())
}

async fn run_server(connection: Connection, params: serde_json::Value) -> Result<()> {
    info!("Received initialization parameters");

    while let Ok(msg) = connection.receiver.recv() {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    info!("Received shutdown request");
                    return Ok(());
                }

                handle_request(&connection, req).await?;
            }
            Message::Response(resp) => {
                handle_response(resp).await?;
            }
            Message::Notification(notification) => {
                handle_notification(&connection, notification).await?;
            }
        }
    }

    Ok(())
}

async fn handle_request(connection: &Connection, req: Request) -> Result<()> {
    match req.method.as_str() {
        _ => {
            let resp = Response::new_err(
                req.id,
                lsp_server::ErrorCode::MethodNotFound as i32,
                format!("Method not found: {}", req.method),
            );

            connection.sender.send(Message::Response(resp))?;
        }
    }

    Ok(())
}

async fn handle_response(resp: Response) -> Result<()> {
    info!("Received response: {:?}", resp);
    Ok(())
}

async fn handle_notification(connection: &Connection, not: lsp_server::Notification) -> Result<()> {
    match not.method.as_str() {
        "textDocument/didOpen" => {
            handle_did_open(connection, not).await?;
        }
        "textDocument/didChange" => {
            handle_did_change(connection, not).await?;
        }
        _ => {
            info!("Unhandled notification: {:?}", not);
        }
    }

    Ok(())
}

async fn handle_did_change(connection: &Connection, not: lsp_server::Notification) -> Result<()> {
    use lsp_types::DidChangeTextDocumentParams;
    info!("Received textDocument/didChange");

    let params: DidChangeTextDocumentParams = serde_json::from_value(not.params)?;
    let uri = params.text_document.uri;

    if let Some(TextDocumentContentChangeEvent { text, .. }) =
        params.content_changes.into_iter().next()
    {
        let result = phprs_parser::parse(&text);

        let diagnostics = match result {
            Ok(_) => Vec::new(),
            Err(errors) => convert_errors_to_diagnostics(&errors, &text),
        };

        publish_diagnostics(connection, uri, diagnostics)?;
    }

    Ok(())
}

async fn handle_did_open(connection: &Connection, not: lsp_server::Notification) -> Result<()> {
    use lsp_types::DidOpenTextDocumentParams;
    info!("Received textDocument/didOpen");

    let params: DidOpenTextDocumentParams = serde_json::from_value(not.params)?;
    let uri = params.text_document.uri;
    let text = params.text_document.text;

    let result = phprs_parser::parse(&text);

    let diagnostics = match result {
        Ok(_) => Vec::new(),
        Err(errors) => convert_errors_to_diagnostics(&errors, &text),
    };

    publish_diagnostics(connection, uri, diagnostics)?;

    Ok(())
}

fn publish_diagnostics(
    connection: &Connection,
    uri: lsp_types::Uri,
    diagnostics: Vec<lsp_types::Diagnostic>,
) -> Result<(), anyhow::Error> {
    let params = PublishDiagnosticsParams {
        uri,
        diagnostics,
        version: None, // or Some(...) if you track doc version
    };

    let not = Notification {
        method: "textDocument/publishDiagnostics".into(),
        params: serde_json::to_value(&params)?,
    };

    connection
        .sender
        .send(lsp_server::Message::Notification(not))?;
    Ok(())
}

fn convert_errors_to_diagnostics(errors: &Vec<Rich<Token>>, content: &str) -> Vec<Diagnostic> {
    errors
        .iter()
        .map(|err| {
            let span = err.span();
            let start_offest = span.start;
            let end_offest = span.end;

            let start_pos = offset_to_position(content, start_offest);
            let end_pos = offset_to_position(content, end_offest);

            Diagnostic {
                range: Range {
                    start: start_pos,
                    end: end_pos,
                },
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!("{}", err),
                ..Diagnostic::default()
            }
        })
        .collect()
}

fn offset_to_position(source: &str, offset: usize) -> Position {
    // A simple line/column calculator:
    // Count newlines up to `offset`.
    // If your file is big, consider a more efficient approach (e.g. a line index).

    let mut line = 0;
    let mut col = 0;
    let mut curr_offset = 0;

    for ch in source.chars() {
        if curr_offset == offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
        curr_offset += ch.len_utf8();
    }

    Position {
        line,
        character: col,
    }
}
