//! Zed extension: registers the SpinDoctor Qwerty stdio MCP server as a context server.
//! The server is the bundled standard-library-only Python engine (engine/scripts/spindoctor_mcp.py).
//! Zed extensions run as WebAssembly and cannot read environment variables or their own install folder,
//! so the engine location comes from the user's Zed settings (context_servers.spindoctor-qwerty.settings).
// SpinDoctor Qwerty 1.1.0 - Zed extension
// Copyright (C) 2026 RUSSELL PHILIP SMITHSON <russellsmithson77@outlook.com>
// SPDX-License-Identifier: GPL-3.0-or-later
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General
// Public License as published by the Free Software Foundation, either version 3 of the License, or (at your
// option) any later version. It is distributed WITHOUT ANY WARRANTY; see the LICENSE file for details.
use zed_extension_api::{
    self as zed, serde_json, settings::ContextServerSettings, ContextServerConfiguration, ContextServerId, Project, Result,
};

struct SpinDoctorQwerty;

const SERVER_ID: &str = "spindoctor-qwerty";
const LAUNCHER: &str = "spindoctor_mcp.py";

/// True for "/abs/path", "C:/x", "C:\\x" and UNC "\\\\server\\share" paths.
fn is_absolute(p: &str) -> bool {
    let b = p.as_bytes();
    p.starts_with('/') || p.starts_with("\\\\") || (b.len() > 2 && b[0].is_ascii_alphabetic() && b[1] == b':' && (b[2] == b'/' || b[2] == b'\\'))
}

fn setting(s: &serde_json::Value, key: &str) -> Option<String> {
    s.get(key).and_then(|v| v.as_str()).map(|v| v.trim().to_string()).filter(|v| !v.is_empty())
}

/// Validate the configured engine launcher path (absolute, points at spindoctor_mcp.py, no control characters).
fn engine_path(s: &serde_json::Value) -> Result<String> {
    let engine = setting(s, "engine").ok_or_else(|| {
        "SpinDoctor Qwerty: set context_servers.spindoctor-qwerty.settings.engine to the absolute path of engine/scripts/spindoctor_mcp.py inside this repository (see README.md).".to_string()
    })?;
    if engine.chars().any(|c| c.is_control()) {
        return Err("SpinDoctor Qwerty: the engine path contains control characters.".to_string());
    }
    if !is_absolute(&engine) {
        return Err(format!("SpinDoctor Qwerty: the engine path must be absolute, got '{engine}'."));
    }
    let file = engine.rsplit(|c| c == '/' || c == '\\').next().unwrap_or("");
    if file != LAUNCHER {
        return Err(format!("SpinDoctor Qwerty: the engine path must end with {LAUNCHER}, got '{file}'."));
    }
    Ok(engine)
}

impl zed::Extension for SpinDoctorQwerty {
    fn new() -> Self {
        SpinDoctorQwerty
    }

    fn context_server_command(&mut self, _id: &ContextServerId, project: &Project) -> Result<zed::Command> {
        let settings = ContextServerSettings::for_project(SERVER_ID, project)?;
        // An explicit "command" block in Zed settings always wins.
        if let Some(cmd) = settings.command {
            if let Some(path) = cmd.path {
                return Ok(zed::Command {
                    command: path,
                    args: cmd.arguments.unwrap_or_default(),
                    env: cmd.env.map(|e| e.into_iter().collect()).unwrap_or_default(),
                });
            }
        }
        let s = settings.settings.unwrap_or(serde_json::Value::Null);
        let engine = engine_path(&s)?;
        let python = setting(&s, "python").unwrap_or_else(|| {
            match zed::current_platform().0 {
                zed::Os::Windows => "python",
                _ => "python3",
            }
            .to_string()
        });
        // UTF-8 stdio everywhere (Windows consoles default to a legacy code page); no .pyc files next to the engine.
        let mut env = vec![("PYTHONUTF8".to_string(), "1".to_string()), ("PYTHONDONTWRITEBYTECODE".to_string(), "1".to_string())];
        if let Some(ws) = setting(&s, "workspace") {
            env.push(("SPINDOCTOR_WORKSPACE".to_string(), ws));
        }
        Ok(zed::Command { command: python, args: vec![engine], env })
    }

    fn context_server_configuration(&mut self, _id: &ContextServerId, _project: &Project) -> Result<Option<ContextServerConfiguration>> {
        Ok(Some(ContextServerConfiguration {
            installation_instructions: include_str!("../configuration/installation_instructions.md").to_string(),
            settings_schema: include_str!("../configuration/settings_schema.json").to_string(),
            default_settings: include_str!("../configuration/default_settings.jsonc").to_string(),
        }))
    }
}

zed::register_extension!(SpinDoctorQwerty);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_paths() {
        assert!(is_absolute("/home/me/spindoctor-qwerty-zed/engine/scripts/spindoctor_mcp.py"));
        assert!(is_absolute("C:/Users/me/spindoctor-qwerty-zed/engine/scripts/spindoctor_mcp.py"));
        assert!(is_absolute("C:\\x\\spindoctor_mcp.py"));
        assert!(is_absolute("\\\\server\\share\\spindoctor_mcp.py"));
        assert!(!is_absolute("engine/scripts/spindoctor_mcp.py"));
        assert!(!is_absolute("~/x/spindoctor_mcp.py"));
    }

    #[test]
    fn engine_validation() {
        let ok = serde_json::json!({"engine": " /a/engine/scripts/spindoctor_mcp.py "});
        assert_eq!(engine_path(&ok).unwrap(), "/a/engine/scripts/spindoctor_mcp.py");
        assert!(engine_path(&serde_json::json!({})).is_err());
        assert!(engine_path(&serde_json::json!({"engine": ""})).is_err());
        assert!(engine_path(&serde_json::json!({"engine": "relative/spindoctor_mcp.py"})).is_err());
        assert!(engine_path(&serde_json::json!({"engine": "/a/other.py"})).is_err());
        assert!(engine_path(&serde_json::json!({"engine": "/a/spindoctor_mcp.py\n"})).is_ok()); // trimmed
        assert!(engine_path(&serde_json::json!({"engine": "/a\u{0007}/spindoctor_mcp.py"})).is_err());
    }
}
