use serde_json::{Value, json};
use std::process::{Child, Stdio};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

pub struct LspClient {
    child: Child,
    stdin: tokio::process::ChildStdin,
    stdout: tokio::process::ChildStdout,
}

impl LspClient {
    pub async fn start() -> anyhow::Result<Self> {
        let mut child = Command::new("rust-analyzer")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        Ok(Self {
            stdin: child.stdin.take().unwrap(),
            stdout: child.stdout.take().unwrap(),
            child,
        })
    }

    pub async fn send(&mut self, value: Value) -> anyhow::Result<()> {
        let body = value.to_string();
        let msg = format!("Content-Length: {}\r\n\r\n{}", body.len(), body);
        self.stdin.write_all(msg.as_bytes()).await?;
        Ok(())
    }

    pub async fn read(&mut self) -> anyhow::Result<Option<Value>> {
        let mut buf = Vec::new();
        self.stdout.read_buf(&mut buf).await?;
        if buf.is_empty() {
            return Ok(None);
        }

        let text = String::from_utf8_lossy(&buf);
        if let Some(idx) = text.find("\r\n\r\n") {
            let json = &text[idx + 4..];
            return Ok(Some(serde_json::from_str(json)?));
        }
        Ok(None)
    }
}
