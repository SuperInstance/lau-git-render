//! # lau-git-render
//!
//! The rendering layer for git-native agents — the A2A/A2UI bridge that takes
//! the internal state of a git-agent repo and renders it for different consumers.
//!
//! Agents get structured A2A protocol. Humans get A2UI renderings: dashboard,
//! terminal, game engine, Telegram, whatever fits.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

// ── Errors ──────────────────────────────────────────────────────────────────

/// Errors that can occur during rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RenderError {
    /// Failed to capture a repo snapshot.
    SnapshotFailed(String),
    /// No renderer registered for the requested format.
    NoRendererFor(RenderFormat),
    /// Rendering itself failed.
    RenderFailed(String),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SnapshotFailed(msg) => write!(f, "snapshot failed: {msg}"),
            Self::NoRendererFor(fmt) => write!(f, "no renderer registered for {fmt:?}"),
            Self::RenderFailed(msg) => write!(f, "render failed: {msg}"),
        }
    }
}

impl std::error::Error for RenderError {}

// ── Agent Mode ──────────────────────────────────────────────────────────────

/// The current mode of the agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentMode {
    Bootstrap,
    Idle,
    Executing,
    Error,
    Offline,
}

impl fmt::Display for AgentMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bootstrap => write!(f, "bootstrap"),
            Self::Idle => write!(f, "idle"),
            Self::Executing => write!(f, "executing"),
            Self::Error => write!(f, "error"),
            Self::Offline => write!(f, "offline"),
        }
    }
}

// ── Snapshot types ──────────────────────────────────────────────────────────

/// A snapshot of a room within the repo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomSnapshot {
    pub id: String,
    pub gravity: f64,
    pub alert: String,
    pub tile_count: u32,
}

/// A snapshot of an ensign (sub-agent).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsignSnapshot {
    pub id: String,
    pub model: String,
    pub status: String,
    pub room: Option<String>,
}

/// Summary of tiles in the repo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileSummary {
    pub total: u64,
    pub active: u64,
    pub by_type: HashMap<String, u64>,
}

/// A snapshot of a correlation between two rooms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationSnapshot {
    pub room_a: String,
    pub room_b: String,
    pub strength: f64,
    pub spline_type: String,
}

/// Conservation budget snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConservationSnapshot {
    pub budget: f64,
    pub spent: f64,
    pub remaining: f64,
}

/// A point-in-time view of the entire repo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoSnapshot {
    pub commit_sha: String,
    pub branch: String,
    pub timestamp: u64,
    pub state: AgentMode,
    pub identity: Option<String>,
    pub rooms: Vec<RoomSnapshot>,
    pub ensigns: Vec<EnsignSnapshot>,
    pub tiles: TileSummary,
    pub correlations: Vec<CorrelationSnapshot>,
    pub provenance_count: usize,
    pub inbox_count: usize,
    pub conservation: ConservationSnapshot,
}

impl RepoSnapshot {
    /// Create a snapshot from a repo path (placeholder — real impl reads git).
    pub fn from_repo(_repo_path: &str) -> Result<Self, RenderError> {
        // In a real implementation, this would read git refs, parse state files, etc.
        // For now returns a stub that demonstrates the shape.
        Ok(Self {
            commit_sha: String::new(),
            branch: String::new(),
            timestamp: 0,
            state: AgentMode::Idle,
            identity: None,
            rooms: Vec::new(),
            ensigns: Vec::new(),
            tiles: TileSummary {
                total: 0,
                active: 0,
                by_type: HashMap::new(),
            },
            correlations: Vec::new(),
            provenance_count: 0,
            inbox_count: 0,
            conservation: ConservationSnapshot {
                budget: 0.0,
                spent: 0.0,
                remaining: 0.0,
            },
        })
    }
}

// ── Render context ──────────────────────────────────────────────────────────

/// The context for a render operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderContext {
    pub repo_path: String,
    pub snapshot: RepoSnapshot,
}

impl RenderContext {
    /// Build a render context from a repo path.
    pub fn from_repo(repo_path: &str) -> Result<Self, RenderError> {
        Ok(Self {
            repo_path: repo_path.to_string(),
            snapshot: RepoSnapshot::from_repo(repo_path)?,
        })
    }
}

// ── Additional render data types ────────────────────────────────────────────

/// Data for rendering a single tile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileRenderData {
    pub id: String,
    pub tile_type: String,
    pub room: Option<String>,
    pub status: String,
    pub content_preview: String,
    pub age: u64,
}

/// High-level repo status summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoStatus {
    pub mode: String,
    pub rooms: u32,
    pub ensigns: u32,
    pub tiles: u64,
    pub conservation_remaining: f64,
    pub uptime: u64,
}

/// A provenance entry for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceEntry {
    pub id: String,
    pub intent: String,
    pub model: String,
    pub timestamp: u64,
    pub alternatives: Vec<ProvenanceAlternative>,
}

/// A single alternative within a provenance entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceAlternative {
    pub label: String,
    pub chosen: bool,
}

// ── Render format ───────────────────────────────────────────────────────────

/// The kind of rendering to produce.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RenderFormat {
    Terminal,
    Dashboard,
    GameEngine,
    Telegram,
    A2A,
    Json,
    Markdown,
    Voice,
}

impl fmt::Display for RenderFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Terminal => write!(f, "terminal"),
            Self::Dashboard => write!(f, "dashboard"),
            Self::GameEngine => write!(f, "game-engine"),
            Self::Telegram => write!(f, "telegram"),
            Self::A2A => write!(f, "a2a"),
            Self::Json => write!(f, "json"),
            Self::Markdown => write!(f, "markdown"),
            Self::Voice => write!(f, "voice"),
        }
    }
}

// ── Renderer trait ──────────────────────────────────────────────────────────

/// The core rendering abstraction. Each renderer produces output for a specific
/// consumer type from the same underlying state.
pub trait Renderer: Send + Sync {
    /// Which format this renderer handles.
    fn format(&self) -> RenderFormat;

    /// Render the full snapshot.
    fn render_snapshot(&self, ctx: &RenderContext) -> Result<String, RenderError>;

    /// Render a single room.
    fn render_room(&self, room: &RoomSnapshot) -> Result<String, RenderError>;

    /// Render a single ensign.
    fn render_ensign(&self, ensign: &EnsignSnapshot) -> Result<String, RenderError>;

    /// Render correlations.
    fn render_correlations(&self, corrs: &[CorrelationSnapshot]) -> Result<String, RenderError>;

    /// Render a provenance entry.
    fn render_provenance(&self, entry: &ProvenanceEntry) -> Result<String, RenderError>;

    /// Render a tile.
    fn render_tile(&self, tile: &TileRenderData) -> Result<String, RenderError>;

    /// Render repo status.
    fn render_status(&self, status: &RepoStatus) -> Result<String, RenderError>;
}

// ── Terminal Renderer ───────────────────────────────────────────────────────

/// ASCII art renderer for terminal users. Uses box-drawing characters and ANSI
/// color codes.
pub struct TerminalRenderer;

impl TerminalRenderer {
    pub fn new() -> Self {
        Self
    }

    fn alert_color(alert: &str) -> &'static str {
        match alert {
            "green" => "\x1b[32m",
            "yellow" => "\x1b[33m",
            "red" => "\x1b[31m",
            _ => "\x1b[0m",
        }
    }

    fn reset() -> &'static str {
        "\x1b[0m"
    }

    /// Render a 2D ASCII grid of rooms with ensigns.
    pub fn render_grid(&self, rooms: &[RoomSnapshot], ensigns: &[EnsignSnapshot]) -> String {
        let mut out = String::from("╔══════════════════════════════════════╗\n");
        out.push_str("║          ROOM LAYOUT GRID           ║\n");
        out.push_str("╠══════════════════════════════════════╣\n");

        for room in rooms {
            let color = Self::alert_color(&room.alert);
            let reset = Self::reset();
            out.push_str(&format!(
                "║ {}┌─{:<16}─┐{} ║\n",
                color, room.id, reset
            ));
            out.push_str(&format!(
                "║ {}│ gravity: {:>7.2}  │{} ║\n",
                color, room.gravity, reset
            ));
            out.push_str(&format!(
                "║ {}│ alert: {:<9}  │{} ║\n",
                color, room.alert, reset
            ));
            out.push_str(&format!(
                "║ {}│ tiles: {:<9}  │{} ║\n",
                color, room.tile_count, reset
            ));

            // Show ensigns in this room
            let room_ensigns: Vec<_> = ensigns
                .iter()
                .filter(|e| e.room.as_deref() == Some(&room.id))
                .collect();
            for ensign in &room_ensigns {
                out.push_str(&format!(
                    "║ {}│  ↳ {} ({}){} ║\n",
                    color, ensign.id, ensign.model, reset
                ));
            }
            out.push_str(&format!("║ {}└──────────────────┘{} ║\n", color, reset));
        }
        out.push_str("╚══════════════════════════════════════╝\n");
        out
    }

    /// Render an ASCII timeline of recent tiles.
    pub fn render_timeline(&self, tiles: &[TileRenderData]) -> String {
        let mut out = String::from("─── TILE TIMELINE ───\n\n");
        for (i, tile) in tiles.iter().enumerate() {
            let connector = if i < tiles.len() - 1 {
                "├──"
            } else {
                "└──"
            };
            out.push_str(&format!(
                "{} [{:>5}] {} ({}) @ {} age={}\n",
                connector,
                tile.id,
                tile.tile_type,
                tile.status,
                tile.room.as_deref().unwrap_or("orphan"),
                tile.age
            ));
            if !tile.content_preview.is_empty() {
                let preview: String = tile.content_preview.chars().take(40).collect();
                out.push_str(&format!("│         \"{}\"\n", preview));
            }
        }
        out
    }
}

impl Default for TerminalRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for TerminalRenderer {
    fn format(&self) -> RenderFormat {
        RenderFormat::Terminal
    }

    fn render_snapshot(&self, ctx: &RenderContext) -> Result<String, RenderError> {
        let mut out = String::new();
        let s = &ctx.snapshot;

        out.push_str("╔══════════════════════════════════════════╗\n");
        out.push_str("║         REPO SNAPSHOT — TERMINAL        ║\n");
        out.push_str("╠══════════════════════════════════════════╣\n");
        out.push_str(&format!("║ branch:  {:<30} ║\n", s.branch));
        out.push_str(&format!(
            "║ commit:  {:<30} ║\n",
            &s.commit_sha[..s.commit_sha.len().min(30)]
        ));
        out.push_str(&format!("║ mode:    {:<30} ║\n", s.state));
        out.push_str(&format!(
            "║ time:    {:<30} ║\n",
            s.timestamp
        ));
        out.push_str("╠══════════════════════════════════════════╣\n");
        out.push_str(&format!(
            "║ rooms: {}  ensigns: {}  tiles: {} ║\n",
            s.rooms.len(),
            s.ensigns.len(),
            s.tiles.total
        ));
        out.push_str(&format!(
            "║ correlations: {}  provenance: {} ║\n",
            s.correlations.len(),
            s.provenance_count
        ));
        out.push_str("╚══════════════════════════════════════════╝\n\n");

        out.push_str(&self.render_grid(&s.rooms, &s.ensigns));

        if !s.correlations.is_empty() {
            out.push_str("\n─── CORRELATIONS ───\n");
            out.push_str(&self.render_correlations(&s.correlations)?);
        }

        Ok(out)
    }

    fn render_room(&self, room: &RoomSnapshot) -> Result<String, RenderError> {
        let color = Self::alert_color(&room.alert);
        let reset = Self::reset();
        Ok(format!(
            "{}┌──────────────────┐\n\
             │ Room: {:<12} │\n\
             │ Gravity: {:>8.2} │\n\
             │ Alert: {:<11} │\n\
             │ Tiles: {:<11} │\n\
             └──────────────────┘{}",
            color, room.id, room.gravity, room.alert, room.tile_count, reset
        ))
    }

    fn render_ensign(&self, ensign: &EnsignSnapshot) -> Result<String, RenderError> {
        let room_str = ensign
            .room
            .as_deref()
            .unwrap_or("unassigned");
        Ok(format!(
            "┌──────────────────┐\n\
             │ Ensign: {:<10} │\n\
             │ Model: {:<10} │\n\
             │ Status: {:<9} │\n\
             │ Room: {:<11} │\n\
             └──────────────────┘",
            ensign.id, ensign.model, ensign.status, room_str
        ))
    }

    fn render_correlations(&self, corrs: &[CorrelationSnapshot]) -> Result<String, RenderError> {
        let mut out = String::new();
        for c in corrs {
            out.push_str(&format!(
                "  {} ──{} ({:.2})── {} {}\n",
                c.room_a,
                match c.spline_type.as_str() {
                    "causal" => "▸",
                    "inverse" => "◂",
                    "bidirectional" => "▸◂",
                    _ => "─",
                },
                c.strength,
                c.room_b,
                &c.spline_type,
            ));
        }
        Ok(out)
    }

    fn render_provenance(&self, entry: &ProvenanceEntry) -> Result<String, RenderError> {
        let mut out = format!(
            "┌─ Provenance: {} ─┐\n│ intent: {}\n│ model:  {}\n│ alternatives:\n",
            entry.id, entry.intent, entry.model
        );
        for alt in &entry.alternatives {
            let marker = if alt.chosen { "✓" } else { " " };
            out.push_str(&format!("│   {} {}\n", marker, alt.label));
        }
        out.push_str("└────────────────────┘\n");
        Ok(out)
    }

    fn render_tile(&self, tile: &TileRenderData) -> Result<String, RenderError> {
        let room_str = tile.room.as_deref().unwrap_or("orphan");
        Ok(format!(
            "┌─ Tile {} ─────────────┐\n\
             │ type:     {}\n\
             │ room:     {}\n\
             │ status:   {}\n\
             │ age:      {}\n\
             │ preview:  {:.40}\n\
             └────────────────────┘",
            tile.id, tile.tile_type, room_str, tile.status, tile.age, tile.content_preview
        ))
    }

    fn render_status(&self, status: &RepoStatus) -> Result<String, RenderError> {
        Ok(format!(
            "╔═══════════════════════╗\n\
             ║ STATUS: {:<13} ║\n\
             ║ Rooms:      {:>9} ║\n\
             ║ Ensigns:    {:>9} ║\n\
             ║ Tiles:      {:>9} ║\n\
             ║ Budget:  {:>10.1} ║\n\
             ║ Uptime:     {:>9} ║\n\
             ╚═══════════════════════╝",
            status.mode, status.rooms, status.ensigns, status.tiles, status.conservation_remaining, status.uptime
        ))
    }
}

// ── Dashboard Renderer ──────────────────────────────────────────────────────

/// Produces structured widget data for dashboard consumers.
pub struct DashboardRenderer;

impl DashboardRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DashboardRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for DashboardRenderer {
    fn format(&self) -> RenderFormat {
        RenderFormat::Dashboard
    }

    fn render_snapshot(&self, ctx: &RenderContext) -> Result<String, RenderError> {
        let s = &ctx.snapshot;
        let mut widgets: Vec<serde_json::Value> = Vec::new();

        for room in &s.rooms {
            widgets.push(serde_json::json!({
                "type": "room",
                "id": room.id,
                "gravity": room.gravity,
                "alert": room.alert,
                "tiles": room.tile_count,
            }));
        }

        for ensign in &s.ensigns {
            widgets.push(serde_json::json!({
                "type": "ensign",
                "id": ensign.id,
                "model": ensign.model,
                "room": ensign.room,
                "status": ensign.status,
            }));
        }

        widgets.push(serde_json::json!({
            "type": "gauge",
            "id": "conservation",
            "value": s.conservation.remaining,
            "max": s.conservation.budget,
        }));

        for corr in &s.correlations {
            widgets.push(serde_json::json!({
                "type": "correlation",
                "from": corr.room_a,
                "to": corr.room_b,
                "strength": corr.strength,
                "type": corr.spline_type,
            }));
        }

        let output = serde_json::json!({
            "widgets": widgets,
            "layout": "grid",
            "timestamp": s.timestamp,
        });

        serde_json::to_string_pretty(&output).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_room(&self, room: &RoomSnapshot) -> Result<String, RenderError> {
        let widget = serde_json::json!({
            "type": "room",
            "id": room.id,
            "gravity": room.gravity,
            "alert": room.alert,
            "tiles": room.tile_count,
        });
        serde_json::to_string_pretty(&widget).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_ensign(&self, ensign: &EnsignSnapshot) -> Result<String, RenderError> {
        let widget = serde_json::json!({
            "type": "ensign",
            "id": ensign.id,
            "model": ensign.model,
            "room": ensign.room,
            "status": ensign.status,
        });
        serde_json::to_string_pretty(&widget).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_correlations(&self, corrs: &[CorrelationSnapshot]) -> Result<String, RenderError> {
        let widgets: Vec<_> = corrs
            .iter()
            .map(|c| {
                serde_json::json!({
                    "type": "correlation",
                    "from": c.room_a,
                    "to": c.room_b,
                    "strength": c.strength,
                    "spline_type": c.spline_type,
                })
            })
            .collect();
        serde_json::to_string_pretty(&widgets).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_provenance(&self, entry: &ProvenanceEntry) -> Result<String, RenderError> {
        let alts: Vec<_> = entry
            .alternatives
            .iter()
            .map(|a| serde_json::json!({"label": a.label, "chosen": a.chosen}))
            .collect();
        let widget = serde_json::json!({
            "type": "provenance",
            "id": entry.id,
            "intent": entry.intent,
            "model": entry.model,
            "timestamp": entry.timestamp,
            "alternatives": alts,
        });
        serde_json::to_string_pretty(&widget).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_tile(&self, tile: &TileRenderData) -> Result<String, RenderError> {
        let widget = serde_json::json!({
            "type": "tile",
            "id": tile.id,
            "tile_type": tile.tile_type,
            "room": tile.room,
            "status": tile.status,
            "content_preview": tile.content_preview,
            "age": tile.age,
        });
        serde_json::to_string_pretty(&widget).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_status(&self, status: &RepoStatus) -> Result<String, RenderError> {
        let widget = serde_json::json!({
            "type": "status",
            "mode": status.mode,
            "rooms": status.rooms,
            "ensigns": status.ensigns,
            "tiles": status.tiles,
            "conservation_remaining": status.conservation_remaining,
            "uptime": status.uptime,
        });
        serde_json::to_string_pretty(&widget).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }
}

// ── Game Engine Renderer ────────────────────────────────────────────────────

/// Produces scene-graph JSON for Unity/Godot-style consumers.
pub struct GameEngineRenderer;

impl GameEngineRenderer {
    pub fn new() -> Self {
        Self
    }

    /// Assign 3D positions to rooms in a simple layout.
    fn room_position(index: usize, _total: usize) -> [f64; 3] {
        let row = (index / 3) as f64;
        let col = (index % 3) as f64;
        [col * 10.0, 0.0, row * 10.0]
    }
}

impl Default for GameEngineRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for GameEngineRenderer {
    fn format(&self) -> RenderFormat {
        RenderFormat::GameEngine
    }

    fn render_snapshot(&self, ctx: &RenderContext) -> Result<String, RenderError> {
        let s = &ctx.snapshot;
        let mut entities: Vec<serde_json::Value> = Vec::new();
        let mut connections: Vec<serde_json::Value> = Vec::new();

        for (i, room) in s.rooms.iter().enumerate() {
            let pos = Self::room_position(i, s.rooms.len());
            entities.push(serde_json::json!({
                "id": format!("room_{}", room.id),
                "type": "room",
                "position": pos,
                "gravity": room.gravity,
                "alert": room.alert,
            }));
        }

        for ensign in &s.ensigns {
            let parent = ensign
                .room
                .as_deref()
                .map(|r| format!("room_{r}"))
                .unwrap_or_default();
            entities.push(serde_json::json!({
                "id": format!("ensign_{}", ensign.id),
                "type": "ensign",
                "parent": parent,
                "model": ensign.model,
                "state": ensign.status,
            }));
        }

        for corr in &s.correlations {
            connections.push(serde_json::json!({
                "from": format!("room_{}", corr.room_a),
                "to": format!("room_{}", corr.room_b),
                "strength": corr.strength,
                "type": corr.spline_type,
            }));
        }

        let output = serde_json::json!({
            "entities": entities,
            "connections": connections,
            "environment": {
                "lighting": match s.state {
                    AgentMode::Error => "red",
                    AgentMode::Bootstrap => "dim",
                    AgentMode::Offline => "off",
                    _ => "normal",
                },
                "conservation_remaining": s.conservation.remaining,
            },
        });

        serde_json::to_string_pretty(&output).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_room(&self, room: &RoomSnapshot) -> Result<String, RenderError> {
        let entity = serde_json::json!({
            "id": format!("room_{}", room.id),
            "type": "room",
            "position": [0.0, 0.0, 0.0],
            "gravity": room.gravity,
            "alert": room.alert,
        });
        serde_json::to_string_pretty(&entity).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_ensign(&self, ensign: &EnsignSnapshot) -> Result<String, RenderError> {
        let parent = ensign
            .room
            .as_deref()
            .map(|r| format!("room_{r}"))
            .unwrap_or_default();
        let entity = serde_json::json!({
            "id": format!("ensign_{}", ensign.id),
            "type": "ensign",
            "parent": parent,
            "model": ensign.model,
            "state": ensign.status,
        });
        serde_json::to_string_pretty(&entity).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_correlations(&self, corrs: &[CorrelationSnapshot]) -> Result<String, RenderError> {
        let connections: Vec<_> = corrs
            .iter()
            .map(|c| {
                serde_json::json!({
                    "from": format!("room_{}", c.room_a),
                    "to": format!("room_{}", c.room_b),
                    "strength": c.strength,
                    "type": c.spline_type,
                })
            })
            .collect();
        serde_json::to_string_pretty(&connections).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_provenance(&self, entry: &ProvenanceEntry) -> Result<String, RenderError> {
        let entity = serde_json::json!({
            "id": format!("provenance_{}", entry.id),
            "type": "provenance",
            "intent": entry.intent,
            "model": entry.model,
            "timestamp": entry.timestamp,
        });
        serde_json::to_string_pretty(&entity).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_tile(&self, tile: &TileRenderData) -> Result<String, RenderError> {
        let parent = tile
            .room
            .as_deref()
            .map(|r| format!("room_{r}"))
            .unwrap_or_default();
        let entity = serde_json::json!({
            "id": format!("tile_{}", tile.id),
            "type": "tile",
            "parent": parent,
            "tile_type": tile.tile_type,
            "age": tile.age,
        });
        serde_json::to_string_pretty(&entity).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_status(&self, status: &RepoStatus) -> Result<String, RenderError> {
        let entity = serde_json::json!({
            "type": "status",
            "mode": status.mode,
            "rooms": status.rooms,
            "ensigns": status.ensigns,
            "tiles": status.tiles,
            "conservation_remaining": status.conservation_remaining,
        });
        serde_json::to_string_pretty(&entity).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }
}

// ── Telegram Renderer ───────────────────────────────────────────────────────

/// Formatted markdown with emojis, safe for Telegram.
pub struct TelegramRenderer;

impl TelegramRenderer {
    pub fn new() -> Self {
        Self
    }

    fn alert_emoji(alert: &str) -> &'static str {
        match alert {
            "green" => "🟢",
            "yellow" => "🟡",
            "red" => "🔴",
            _ => "⚪",
        }
    }

    fn gravity_emoji(gravity: f64) -> &'static str {
        if gravity < -0.5 {
            "📊"
        } else if gravity > 0.5 {
            "🎨"
        } else {
            "💬"
        }
    }

    fn ensign_role_emoji(id: &str) -> &'static str {
        let id_lower = id.to_lowercase();
        if id_lower.contains("engine") || id_lower.contains("motor") {
            "⚙️"
        } else if id_lower.contains("scien") || id_lower.contains("research") {
            "🔬"
        } else if id_lower.contains("shield") || id_lower.contains("defen") {
            "🛡️"
        } else if id_lower.contains("nav") {
            "📋"
        } else {
            "🤝"
        }
    }

    fn conservation_bar(remaining: f64, budget: f64) -> String {
        if budget <= 0.0 {
            return "N/A".to_string();
        }
        let pct = (remaining / budget * 10.0).round() as i32;
        let filled = pct.clamp(0, 10);
        let empty = 10 - filled;
        format!(
            "[{}{}] {:.0}/{:.0}",
            "█".repeat(filled as usize),
            "░".repeat(empty as usize),
            remaining,
            budget
        )
    }
}

impl Default for TelegramRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for TelegramRenderer {
    fn format(&self) -> RenderFormat {
        RenderFormat::Telegram
    }

    fn render_snapshot(&self, ctx: &RenderContext) -> Result<String, RenderError> {
        let s = &ctx.snapshot;
        let mut out = String::new();

        out.push_str(&format!("🚀 **Repo Status** — `{}`\n", s.branch));
        out.push_str(&format!(
            "Commit: `{}`\n",
            &s.commit_sha[..s.commit_sha.len().min(8)]
        ));
        out.push_str(&format!("Mode: {} | Tiles: {}\n\n", s.state, s.tiles.total));

        // Rooms
        if !s.rooms.is_empty() {
            out.push_str("**Rooms:**\n");
            for room in &s.rooms {
                out.push_str(&self.render_room(room)?);
                out.push('\n');
            }
            out.push('\n');
        }

        // Ensigns
        if !s.ensigns.is_empty() {
            out.push_str("**Ensigns:**\n");
            for ensign in &s.ensigns {
                out.push_str(&self.render_ensign(ensign)?);
                out.push('\n');
            }
            out.push('\n');
        }

        // Correlations
        if !s.correlations.is_empty() {
            out.push_str(&self.render_correlations(&s.correlations)?);
            out.push('\n');
        }

        // Conservation
        out.push_str(&format!(
            "💰 Conservation: {}\n",
            Self::conservation_bar(s.conservation.remaining, s.conservation.budget)
        ));

        Ok(out)
    }

    fn render_room(&self, room: &RoomSnapshot) -> Result<String, RenderError> {
        let alert_e = Self::alert_emoji(&room.alert);
        let grav_e = Self::gravity_emoji(room.gravity);
        Ok(format!(
            "  {} {} `{}` — gravity {:.2}, {} tiles {}",
            alert_e, grav_e, room.id, room.gravity, room.tile_count, room.alert
        ))
    }

    fn render_ensign(&self, ensign: &EnsignSnapshot) -> Result<String, RenderError> {
        let role_e = Self::ensign_role_emoji(&ensign.id);
        let room_str = ensign
            .room
            .as_deref()
            .unwrap_or("unassigned");
        Ok(format!(
            "  {} **{}** ({}) — {} [{}]",
            role_e, ensign.id, ensign.model, ensign.status, room_str
        ))
    }

    fn render_correlations(&self, corrs: &[CorrelationSnapshot]) -> Result<String, RenderError> {
        let mut out = String::from("**Correlations:**\n");
        for c in corrs {
            out.push_str(&format!(
                "  🔗 {} ↔ {} ({:.2}, {})\n",
                c.room_a, c.room_b, c.strength, c.spline_type
            ));
        }
        Ok(out)
    }

    fn render_provenance(&self, entry: &ProvenanceEntry) -> Result<String, RenderError> {
        let mut out = format!("📜 **{}** — {}\n", entry.id, entry.intent);
        out.push_str(&format!("  Model: {}\n", entry.model));
        for alt in &entry.alternatives {
            let marker = if alt.chosen { "✅" } else { "⬜" };
            out.push_str(&format!("  {} {}\n", marker, alt.label));
        }
        Ok(out)
    }

    fn render_tile(&self, tile: &TileRenderData) -> Result<String, RenderError> {
        let room_str = tile.room.as_deref().unwrap_or("orphan");
        Ok(format!(
            "🧩 **{}** — {} [{}] @ {}\n  _{}_",
            tile.id, tile.tile_type, tile.status, room_str, tile.content_preview
        ))
    }

    fn render_status(&self, status: &RepoStatus) -> Result<String, RenderError> {
        Ok(format!(
            "🚀 **{}** — {} rooms, {} ensigns, {} tiles\n💰 Budget: {:.0} remaining",
            status.mode, status.rooms, status.ensigns, status.tiles, status.conservation_remaining
        ))
    }
}

// ── A2A Renderer ────────────────────────────────────────────────────────────

/// Structured JSON protocol for agent-to-agent communication.
pub struct A2ARenderer {
    sender: String,
}

impl A2ARenderer {
    pub fn new(sender: &str) -> Self {
        Self {
            sender: sender.to_string(),
        }
    }

    fn wrap(&self, msg_type: &str, payload: serde_json::Value, timestamp: u64) -> serde_json::Value {
        serde_json::json!({
            "protocol": "a2a/v1",
            "type": msg_type,
            "sender": self.sender,
            "timestamp": timestamp,
            "payload": payload,
        })
    }
}

impl Renderer for A2ARenderer {
    fn format(&self) -> RenderFormat {
        RenderFormat::A2A
    }

    fn render_snapshot(&self, ctx: &RenderContext) -> Result<String, RenderError> {
        let payload = serde_json::to_value(&ctx.snapshot)
            .map_err(|e| RenderError::RenderFailed(e.to_string()))?;
        let msg = self.wrap("RepoStatus", payload, ctx.snapshot.timestamp);
        serde_json::to_string_pretty(&msg).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_room(&self, room: &RoomSnapshot) -> Result<String, RenderError> {
        let payload = serde_json::to_value(room)
            .map_err(|e| RenderError::RenderFailed(e.to_string()))?;
        let msg = self.wrap("RoomUpdate", payload, 0);
        serde_json::to_string_pretty(&msg).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_ensign(&self, ensign: &EnsignSnapshot) -> Result<String, RenderError> {
        let payload = serde_json::to_value(ensign)
            .map_err(|e| RenderError::RenderFailed(e.to_string()))?;
        let msg = self.wrap("EnsignUpdate", payload, 0);
        serde_json::to_string_pretty(&msg).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_correlations(&self, corrs: &[CorrelationSnapshot]) -> Result<String, RenderError> {
        let payload = serde_json::json!(corrs);
        let msg = self.wrap("CorrelationDetected", payload, 0);
        serde_json::to_string_pretty(&msg).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_provenance(&self, entry: &ProvenanceEntry) -> Result<String, RenderError> {
        let payload = serde_json::to_value(entry)
            .map_err(|e| RenderError::RenderFailed(e.to_string()))?;
        let msg = self.wrap("ProvenanceEntry", payload, entry.timestamp);
        serde_json::to_string_pretty(&msg).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_tile(&self, tile: &TileRenderData) -> Result<String, RenderError> {
        let payload = serde_json::to_value(tile)
            .map_err(|e| RenderError::RenderFailed(e.to_string()))?;
        let msg = self.wrap("TileEvent", payload, 0);
        serde_json::to_string_pretty(&msg).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_status(&self, status: &RepoStatus) -> Result<String, RenderError> {
        let payload = serde_json::to_value(status)
            .map_err(|e| RenderError::RenderFailed(e.to_string()))?;
        let msg = self.wrap("RepoStatus", payload, 0);
        serde_json::to_string_pretty(&msg).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }
}

// ── JSON Renderer ───────────────────────────────────────────────────────────

/// Raw JSON dump renderer.
pub struct JsonRenderer;

impl JsonRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for JsonRenderer {
    fn format(&self) -> RenderFormat {
        RenderFormat::Json
    }

    fn render_snapshot(&self, ctx: &RenderContext) -> Result<String, RenderError> {
        serde_json::to_string_pretty(&ctx.snapshot)
            .map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_room(&self, room: &RoomSnapshot) -> Result<String, RenderError> {
        serde_json::to_string_pretty(room).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_ensign(&self, ensign: &EnsignSnapshot) -> Result<String, RenderError> {
        serde_json::to_string_pretty(ensign).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_correlations(&self, corrs: &[CorrelationSnapshot]) -> Result<String, RenderError> {
        serde_json::to_string_pretty(corrs).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_provenance(&self, entry: &ProvenanceEntry) -> Result<String, RenderError> {
        serde_json::to_string_pretty(entry).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_tile(&self, tile: &TileRenderData) -> Result<String, RenderError> {
        serde_json::to_string_pretty(tile).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }

    fn render_status(&self, status: &RepoStatus) -> Result<String, RenderError> {
        serde_json::to_string_pretty(status).map_err(|e| RenderError::RenderFailed(e.to_string()))
    }
}

// ── Markdown Renderer ───────────────────────────────────────────────────────

/// Readable markdown document renderer.
pub struct MarkdownRenderer;

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for MarkdownRenderer {
    fn format(&self) -> RenderFormat {
        RenderFormat::Markdown
    }

    fn render_snapshot(&self, ctx: &RenderContext) -> Result<String, RenderError> {
        let s = &ctx.snapshot;
        let mut out = String::new();

        out.push_str("# Repo Snapshot\n\n");
        out.push_str(&format!("**Branch:** `{}`  \n", s.branch));
        out.push_str(&format!(
            "**Commit:** `{}`  \n",
            &s.commit_sha[..s.commit_sha.len().min(8)]
        ));
        out.push_str(&format!("**Mode:** {}  \n", s.state));
        out.push_str(&format!("**Timestamp:** {}  \n\n", s.timestamp));

        out.push_str("## Overview\n\n");
        out.push_str(&format!("- Rooms: {}\n", s.rooms.len()));
        out.push_str(&format!("- Ensigns: {}\n", s.ensigns.len()));
        out.push_str(&format!("- Tiles: {} ({} active)\n", s.tiles.total, s.tiles.active));
        out.push_str(&format!("- Correlations: {}\n", s.correlations.len()));
        out.push_str(&format!("- Provenance entries: {}\n", s.provenance_count));
        out.push_str(&format!(
            "- Conservation: {:.1} / {:.1}\n\n",
            s.conservation.remaining, s.conservation.budget
        ));

        if !s.rooms.is_empty() {
            out.push_str("## Rooms\n\n");
            for room in &s.rooms {
                out.push_str(&self.render_room(room)?);
                out.push('\n');
            }
        }

        if !s.ensigns.is_empty() {
            out.push_str("## Ensigns\n\n");
            for ensign in &s.ensigns {
                out.push_str(&self.render_ensign(ensign)?);
                out.push('\n');
            }
        }

        if !s.correlations.is_empty() {
            out.push_str("## Correlations\n\n");
            out.push_str(&self.render_correlations(&s.correlations)?);
            out.push('\n');
        }

        out.push_str("## Conservation\n\n");
        out.push_str(&format!(
            "- Budget: {:.1}\n- Spent: {:.1}\n- Remaining: {:.1}\n",
            s.conservation.budget, s.conservation.spent, s.conservation.remaining
        ));

        Ok(out)
    }

    fn render_room(&self, room: &RoomSnapshot) -> Result<String, RenderError> {
        Ok(format!(
            "### Room: {}\n\n- Gravity: {:.2}\n- Alert: {}\n- Tiles: {}\n\n",
            room.id, room.gravity, room.alert, room.tile_count
        ))
    }

    fn render_ensign(&self, ensign: &EnsignSnapshot) -> Result<String, RenderError> {
        let room_str = ensign.room.as_deref().unwrap_or("unassigned");
        Ok(format!(
            "### Ensign: {}\n\n- Model: {}\n- Status: {}\n- Room: {}\n\n",
            ensign.id, ensign.model, ensign.status, room_str
        ))
    }

    fn render_correlations(&self, corrs: &[CorrelationSnapshot]) -> Result<String, RenderError> {
        let mut out = String::new();
        for c in corrs {
            out.push_str(&format!(
                "- **{} ↔ {}**: strength {:.2}, type {}\n",
                c.room_a, c.room_b, c.strength, c.spline_type
            ));
        }
        Ok(out)
    }

    fn render_provenance(&self, entry: &ProvenanceEntry) -> Result<String, RenderError> {
        let mut out = format!(
            "### Provenance: {}\n\n- Intent: {}\n- Model: {}\n- Alternatives:\n",
            entry.id, entry.intent, entry.model
        );
        for alt in &entry.alternatives {
            let marker = if alt.chosen { "✓" } else { " " };
            out.push_str(&format!("  - [{}] {}\n", marker, alt.label));
        }
        Ok(out)
    }

    fn render_tile(&self, tile: &TileRenderData) -> Result<String, RenderError> {
        let room_str = tile.room.as_deref().unwrap_or("orphan");
        Ok(format!(
            "### Tile: {}\n\n- Type: {}\n- Room: {}\n- Status: {}\n- Age: {}\n- Preview: {}\n\n",
            tile.id, tile.tile_type, room_str, tile.status, tile.age, tile.content_preview
        ))
    }

    fn render_status(&self, status: &RepoStatus) -> Result<String, RenderError> {
        Ok(format!(
            "# Status\n\n- Mode: {}\n- Rooms: {}\n- Ensigns: {}\n- Tiles: {}\n- Conservation remaining: {:.1}\n- Uptime: {}\n",
            status.mode, status.rooms, status.ensigns, status.tiles, status.conservation_remaining, status.uptime
        ))
    }
}

// ── Voice Renderer ──────────────────────────────────────────────────────────

/// Speakable text renderer — plain text, conversational tone, no special characters.
pub struct VoiceRenderer;

impl VoiceRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for VoiceRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for VoiceRenderer {
    fn format(&self) -> RenderFormat {
        RenderFormat::Voice
    }

    fn render_snapshot(&self, ctx: &RenderContext) -> Result<String, RenderError> {
        let s = &ctx.snapshot;
        let mut parts = Vec::new();

        parts.push(format!(
            "Repository {} is in {} mode on branch {}.",
            s.identity.as_deref().unwrap_or("unknown"),
            s.state,
            s.branch
        ));

        if !s.rooms.is_empty() {
            parts.push(format!("There are {} rooms.", s.rooms.len()));
            for room in &s.rooms {
                parts.push(self.render_room(room)?.trim().to_string());
            }
        }

        if !s.ensigns.is_empty() {
            parts.push(format!("{} ensigns are active.", s.ensigns.len()));
            for ensign in &s.ensigns {
                parts.push(self.render_ensign(ensign)?.trim().to_string());
            }
        }

        if !s.correlations.is_empty() {
            parts.push(self.render_correlations(&s.correlations)?.trim().to_string());
        }

        parts.push(format!(
            "Conservation budget: {:.0} remaining out of {:.0}.",
            s.conservation.remaining, s.conservation.budget
        ));

        Ok(parts.join(" "))
    }

    fn render_room(&self, room: &RoomSnapshot) -> Result<String, RenderError> {
        let alert_desc = match room.alert.as_str() {
            "green" => "nominal",
            "yellow" => "at yellow alert",
            "red" => "at red alert",
            other => other,
        };
        let grav_desc = if room.gravity < -0.5 {
            "precise"
        } else if room.gravity > 0.5 {
            "creative"
        } else {
            "balanced"
        };
        Ok(format!(
            "The {} room is {} with a {} gravity of {:.2} and {} tiles.",
            room.id, alert_desc, grav_desc, room.gravity, room.tile_count
        ))
    }

    fn render_ensign(&self, ensign: &EnsignSnapshot) -> Result<String, RenderError> {
        let room_str = ensign
            .room
            .as_deref()
            .unwrap_or("unassigned");
        Ok(format!(
            "Ensign {} running {} is {} in the {} room.",
            ensign.id, ensign.model, ensign.status, room_str
        ))
    }

    fn render_correlations(&self, corrs: &[CorrelationSnapshot]) -> Result<String, RenderError> {
        let mut parts = Vec::new();
        for c in corrs {
            parts.push(format!(
                "There is a {} correlation between {} and {} with strength {:.2}.",
                c.spline_type, c.room_a, c.room_b, c.strength
            ));
        }
        Ok(parts.join(" "))
    }

    fn render_provenance(&self, entry: &ProvenanceEntry) -> Result<String, RenderError> {
        let alts: Vec<String> = entry
            .alternatives
            .iter()
            .map(|a| {
                if a.chosen {
                    format!("chose {}", a.label)
                } else {
                    format!("considered {}", a.label)
                }
            })
            .collect();
        Ok(format!(
            "Decision {}: intent was {}. Model {}. Options were: {}.",
            entry.id,
            entry.intent,
            entry.model,
            alts.join(", ")
        ))
    }

    fn render_tile(&self, tile: &TileRenderData) -> Result<String, RenderError> {
        let room_str = tile.room.as_deref().unwrap_or("orphan");
        Ok(format!(
            "Tile {} is a {} tile in the {} room. Status: {}. {}",
            tile.id, tile.tile_type, room_str, tile.status, tile.content_preview
        ))
    }

    fn render_status(&self, status: &RepoStatus) -> Result<String, RenderError> {
        Ok(format!(
            "System is in {} mode with {} rooms, {} ensigns, and {} tiles. Conservation remaining: {:.0}. Uptime: {} seconds.",
            status.mode, status.rooms, status.ensigns, status.tiles, status.conservation_remaining, status.uptime
        ))
    }
}

// ── Render Engine ────────────────────────────────────────────────────────────

/// The render engine picks the right renderer for a given format.
pub struct RenderEngine {
    renderers: HashMap<RenderFormat, Box<dyn Renderer>>,
}

impl RenderEngine {
    /// Create an empty engine.
    pub fn new() -> Self {
        Self {
            renderers: HashMap::new(),
        }
    }

    /// Register a renderer.
    pub fn register(&mut self, renderer: Box<dyn Renderer>) {
        self.renderers.insert(renderer.format(), renderer);
    }

    /// Render a full snapshot in the given format.
    pub fn render(
        &self,
        format: RenderFormat,
        ctx: &RenderContext,
    ) -> Result<String, RenderError> {
        let renderer = self
            .renderers
            .get(&format)
            .ok_or(RenderError::NoRendererFor(format))?;
        renderer.render_snapshot(ctx)
    }

    /// Render a single room in the given format.
    pub fn render_room(
        &self,
        format: RenderFormat,
        room: &RoomSnapshot,
    ) -> Result<String, RenderError> {
        let renderer = self
            .renderers
            .get(&format)
            .ok_or(RenderError::NoRendererFor(format))?;
        renderer.render_room(room)
    }

    /// List available formats.
    pub fn available_formats(&self) -> Vec<RenderFormat> {
        let mut fmts: Vec<_> = self.renderers.keys().copied().collect();
        fmts.sort_by_key(|f| format!("{f:?}"));
        fmts
    }
}

impl Default for RenderEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Build a render engine with all standard renderers registered.
pub fn default_engine() -> RenderEngine {
    let mut engine = RenderEngine::new();
    engine.register(Box::new(TerminalRenderer::new()));
    engine.register(Box::new(DashboardRenderer::new()));
    engine.register(Box::new(GameEngineRenderer::new()));
    engine.register(Box::new(TelegramRenderer::new()));
    engine.register(Box::new(A2ARenderer::new("hermes-construct/default")));
    engine.register(Box::new(JsonRenderer::new()));
    engine.register(Box::new(MarkdownRenderer::new()));
    engine.register(Box::new(VoiceRenderer::new()));
    engine
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Helpers ─────────────────────────────────────────────────────────

    fn test_snapshot() -> RepoSnapshot {
        RepoSnapshot {
            commit_sha: "abc123def456789".to_string(),
            branch: "main".to_string(),
            timestamp: 1234567890,
            state: AgentMode::Executing,
            identity: Some("hermes-construct/abc123".to_string()),
            rooms: vec![
                RoomSnapshot {
                    id: "navigation".to_string(),
                    gravity: -0.3,
                    alert: "yellow".to_string(),
                    tile_count: 142,
                },
                RoomSnapshot {
                    id: "engineering".to_string(),
                    gravity: 0.7,
                    alert: "green".to_string(),
                    tile_count: 88,
                },
            ],
            ensigns: vec![
                EnsignSnapshot {
                    id: "seed-mini".to_string(),
                    model: "Seed-2.0-mini".to_string(),
                    status: "yellow_alert".to_string(),
                    room: Some("navigation".to_string()),
                },
                EnsignSnapshot {
                    id: "shield-guard".to_string(),
                    model: "Shield-1.0".to_string(),
                    status: "active".to_string(),
                    room: Some("engineering".to_string()),
                },
            ],
            tiles: TileSummary {
                total: 230,
                active: 15,
                by_type: {
                    let mut m = HashMap::new();
                    m.insert("observation".to_string(), 142);
                    m.insert("action".to_string(), 88);
                    m
                },
            },
            correlations: vec![CorrelationSnapshot {
                room_a: "engineering".to_string(),
                room_b: "navigation".to_string(),
                strength: 0.87,
                spline_type: "causal".to_string(),
            }],
            provenance_count: 42,
            inbox_count: 3,
            conservation: ConservationSnapshot {
                budget: 10000.0,
                spent: 2500.0,
                remaining: 7500.0,
            },
        }
    }

    fn test_context() -> RenderContext {
        RenderContext {
            repo_path: "/tmp/test-repo".to_string(),
            snapshot: test_snapshot(),
        }
    }

    fn test_room() -> RoomSnapshot {
        RoomSnapshot {
            id: "navigation".to_string(),
            gravity: -0.3,
            alert: "yellow".to_string(),
            tile_count: 142,
        }
    }

    fn test_ensign() -> EnsignSnapshot {
        EnsignSnapshot {
            id: "seed-mini".to_string(),
            model: "Seed-2.0-mini".to_string(),
            status: "yellow_alert".to_string(),
            room: Some("navigation".to_string()),
        }
    }

    fn test_correlation() -> CorrelationSnapshot {
        CorrelationSnapshot {
            room_a: "engineering".to_string(),
            room_b: "navigation".to_string(),
            strength: 0.87,
            spline_type: "causal".to_string(),
        }
    }

    fn test_provenance() -> ProvenanceEntry {
        ProvenanceEntry {
            id: "p001".to_string(),
            intent: "navigate to waypoint".to_string(),
            model: "Seed-2.0-mini".to_string(),
            timestamp: 1234567890,
            alternatives: vec![
                ProvenanceAlternative {
                    label: "direct route".to_string(),
                    chosen: true,
                },
                ProvenanceAlternative {
                    label: "scenic route".to_string(),
                    chosen: false,
                },
            ],
        }
    }

    fn test_tile() -> TileRenderData {
        TileRenderData {
            id: "tile-142".to_string(),
            tile_type: "observation".to_string(),
            room: Some("navigation".to_string()),
            status: "active".to_string(),
            content_preview: "Star field observation at bearing 045".to_string(),
            age: 3,
        }
    }

    fn test_status() -> RepoStatus {
        RepoStatus {
            mode: "executing".to_string(),
            rooms: 5,
            ensigns: 3,
            tiles: 230,
            conservation_remaining: 7500.0,
            uptime: 3600,
        }
    }

    // ── 1. Snapshot / context creation ──────────────────────────────────

    #[test]
    fn test_repo_snapshot_from_repo() {
        let snap = RepoSnapshot::from_repo("/tmp/nonexistent").unwrap();
        assert_eq!(snap.commit_sha, "");
        assert_eq!(snap.state, AgentMode::Idle);
    }

    #[test]
    fn test_render_context_from_repo() {
        let ctx = RenderContext::from_repo("/tmp/nonexistent").unwrap();
        assert_eq!(ctx.repo_path, "/tmp/nonexistent");
        assert_eq!(ctx.snapshot.branch, "");
    }

    #[test]
    fn test_agent_mode_display() {
        assert_eq!(AgentMode::Bootstrap.to_string(), "bootstrap");
        assert_eq!(AgentMode::Idle.to_string(), "idle");
        assert_eq!(AgentMode::Executing.to_string(), "executing");
        assert_eq!(AgentMode::Error.to_string(), "error");
        assert_eq!(AgentMode::Offline.to_string(), "offline");
    }

    #[test]
    fn test_render_format_display() {
        assert_eq!(RenderFormat::Terminal.to_string(), "terminal");
        assert_eq!(RenderFormat::Dashboard.to_string(), "dashboard");
        assert_eq!(RenderFormat::GameEngine.to_string(), "game-engine");
        assert_eq!(RenderFormat::Telegram.to_string(), "telegram");
        assert_eq!(RenderFormat::A2A.to_string(), "a2a");
        assert_eq!(RenderFormat::Json.to_string(), "json");
        assert_eq!(RenderFormat::Markdown.to_string(), "markdown");
        assert_eq!(RenderFormat::Voice.to_string(), "voice");
    }

    // ── 2. Terminal Renderer ────────────────────────────────────────────

    #[test]
    fn test_terminal_format() {
        let r = TerminalRenderer::new();
        assert_eq!(r.format(), RenderFormat::Terminal);
    }

    #[test]
    fn test_terminal_snapshot() {
        let r = TerminalRenderer::new();
        let out = r.render_snapshot(&test_context()).unwrap();
        assert!(out.contains("REPO SNAPSHOT"));
        assert!(out.contains("navigation"));
        assert!(out.contains("engineering"));
    }

    #[test]
    fn test_terminal_room() {
        let r = TerminalRenderer::new();
        let out = r.render_room(&test_room()).unwrap();
        assert!(out.contains("navigation"));
        assert!(out.contains("-0.30"));
    }

    #[test]
    fn test_terminal_ensign() {
        let r = TerminalRenderer::new();
        let out = r.render_ensign(&test_ensign()).unwrap();
        assert!(out.contains("seed-mini"));
        assert!(out.contains("Seed-2.0-mini"));
    }

    #[test]
    fn test_terminal_correlations() {
        let r = TerminalRenderer::new();
        let out = r.render_correlations(&[test_correlation()]).unwrap();
        assert!(out.contains("engineering"));
        assert!(out.contains("navigation"));
        assert!(out.contains("0.87"));
    }

    #[test]
    fn test_terminal_provenance() {
        let r = TerminalRenderer::new();
        let out = r.render_provenance(&test_provenance()).unwrap();
        assert!(out.contains("p001"));
        assert!(out.contains("✓ direct route"));
    }

    #[test]
    fn test_terminal_tile() {
        let r = TerminalRenderer::new();
        let out = r.render_tile(&test_tile()).unwrap();
        assert!(out.contains("tile-142"));
        assert!(out.contains("observation"));
    }

    #[test]
    fn test_terminal_status() {
        let r = TerminalRenderer::new();
        let out = r.render_status(&test_status()).unwrap();
        assert!(out.contains("executing"));
        assert!(out.contains("230"));
    }

    #[test]
    fn test_terminal_grid() {
        let r = TerminalRenderer::new();
        let out = r.render_grid(&[test_room()], &[test_ensign()]);
        assert!(out.contains("ROOM LAYOUT GRID"));
        assert!(out.contains("navigation"));
        assert!(out.contains("seed-mini"));
    }

    #[test]
    fn test_terminal_timeline() {
        let r = TerminalRenderer::new();
        let out = r.render_timeline(&[test_tile()]);
        assert!(out.contains("TILE TIMELINE"));
        assert!(out.contains("tile-142"));
    }

    // ── 3. Dashboard Renderer ───────────────────────────────────────────

    #[test]
    fn test_dashboard_format() {
        let r = DashboardRenderer::new();
        assert_eq!(r.format(), RenderFormat::Dashboard);
    }

    #[test]
    fn test_dashboard_snapshot() {
        let r = DashboardRenderer::new();
        let out = r.render_snapshot(&test_context()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["layout"], "grid");
        assert!(v["widgets"].is_array());
        let widgets = v["widgets"].as_array().unwrap();
        assert!(widgets.len() >= 5); // 2 rooms + 2 ensigns + 1 gauge + 1 corr
    }

    #[test]
    fn test_dashboard_room() {
        let r = DashboardRenderer::new();
        let out = r.render_room(&test_room()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "room");
        assert_eq!(v["id"], "navigation");
    }

    #[test]
    fn test_dashboard_ensign() {
        let r = DashboardRenderer::new();
        let out = r.render_ensign(&test_ensign()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "ensign");
        assert_eq!(v["model"], "Seed-2.0-mini");
    }

    #[test]
    fn test_dashboard_correlations() {
        let r = DashboardRenderer::new();
        let out = r.render_correlations(&[test_correlation()]).unwrap();
        let v: Vec<serde_json::Value> = serde_json::from_str(&out).unwrap();
        assert_eq!(v[0]["strength"], 0.87);
    }

    #[test]
    fn test_dashboard_provenance() {
        let r = DashboardRenderer::new();
        let out = r.render_provenance(&test_provenance()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "provenance");
        assert_eq!(v["alternatives"][0]["chosen"], true);
    }

    #[test]
    fn test_dashboard_tile() {
        let r = DashboardRenderer::new();
        let out = r.render_tile(&test_tile()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "tile");
        assert_eq!(v["age"], 3);
    }

    #[test]
    fn test_dashboard_status() {
        let r = DashboardRenderer::new();
        let out = r.render_status(&test_status()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "status");
        assert_eq!(v["rooms"], 5);
    }

    // ── 4. Game Engine Renderer ─────────────────────────────────────────

    #[test]
    fn test_game_engine_format() {
        let r = GameEngineRenderer::new();
        assert_eq!(r.format(), RenderFormat::GameEngine);
    }

    #[test]
    fn test_game_engine_snapshot() {
        let r = GameEngineRenderer::new();
        let out = r.render_snapshot(&test_context()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        let entities = v["entities"].as_array().unwrap();
        assert!(entities.len() >= 4); // 2 rooms + 2 ensigns
        assert!(v["connections"].is_array());
        assert_eq!(v["environment"]["lighting"], "normal");
    }

    #[test]
    fn test_game_engine_room() {
        let r = GameEngineRenderer::new();
        let out = r.render_room(&test_room()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "room");
        assert_eq!(v["alert"], "yellow");
    }

    #[test]
    fn test_game_engine_ensign() {
        let r = GameEngineRenderer::new();
        let out = r.render_ensign(&test_ensign()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "ensign");
        assert_eq!(v["parent"], "room_navigation");
    }

    #[test]
    fn test_game_engine_correlations() {
        let r = GameEngineRenderer::new();
        let out = r.render_correlations(&[test_correlation()]).unwrap();
        let v: Vec<serde_json::Value> = serde_json::from_str(&out).unwrap();
        assert_eq!(v[0]["from"], "room_engineering");
    }

    #[test]
    fn test_game_engine_provenance() {
        let r = GameEngineRenderer::new();
        let out = r.render_provenance(&test_provenance()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "provenance");
    }

    #[test]
    fn test_game_engine_tile() {
        let r = GameEngineRenderer::new();
        let out = r.render_tile(&test_tile()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "tile");
        assert_eq!(v["parent"], "room_navigation");
    }

    #[test]
    fn test_game_engine_status() {
        let r = GameEngineRenderer::new();
        let out = r.render_status(&test_status()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["tiles"], 230);
    }

    #[test]
    fn test_game_engine_error_lighting() {
        let mut ctx = test_context();
        ctx.snapshot.state = AgentMode::Error;
        let r = GameEngineRenderer::new();
        let out = r.render_snapshot(&ctx).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["environment"]["lighting"], "red");
    }

    #[test]
    fn test_game_engine_bootstrap_lighting() {
        let mut ctx = test_context();
        ctx.snapshot.state = AgentMode::Bootstrap;
        let r = GameEngineRenderer::new();
        let out = r.render_snapshot(&ctx).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["environment"]["lighting"], "dim");
    }

    #[test]
    fn test_room_position() {
        let pos = GameEngineRenderer::room_position(0, 4);
        assert_eq!(pos, [0.0, 0.0, 0.0]);
        let pos = GameEngineRenderer::room_position(1, 4);
        assert_eq!(pos, [10.0, 0.0, 0.0]);
        let pos = GameEngineRenderer::room_position(3, 4);
        assert_eq!(pos, [0.0, 0.0, 10.0]);
    }

    // ── 5. Telegram Renderer ────────────────────────────────────────────

    #[test]
    fn test_telegram_format() {
        let r = TelegramRenderer::new();
        assert_eq!(r.format(), RenderFormat::Telegram);
    }

    #[test]
    fn test_telegram_snapshot() {
        let r = TelegramRenderer::new();
        let out = r.render_snapshot(&test_context()).unwrap();
        assert!(out.contains("Repo Status"));
        assert!(out.contains("🟡"));
        assert!(out.contains("navigation"));
        assert!(out.contains("Conservation"));
    }

    #[test]
    fn test_telegram_room() {
        let r = TelegramRenderer::new();
        let out = r.render_room(&test_room()).unwrap();
        assert!(out.contains("🟡"));
        assert!(out.contains("navigation"));
    }

    #[test]
    fn test_telegram_ensign() {
        let r = TelegramRenderer::new();
        let out = r.render_ensign(&test_ensign()).unwrap();
        assert!(out.contains("seed-mini"));
    }

    #[test]
    fn test_telegram_correlations() {
        let r = TelegramRenderer::new();
        let out = r.render_correlations(&[test_correlation()]).unwrap();
        assert!(out.contains("🔗"));
        assert!(out.contains("0.87"));
    }

    #[test]
    fn test_telegram_provenance() {
        let r = TelegramRenderer::new();
        let out = r.render_provenance(&test_provenance()).unwrap();
        assert!(out.contains("📜"));
        assert!(out.contains("✅ direct route"));
    }

    #[test]
    fn test_telegram_tile() {
        let r = TelegramRenderer::new();
        let out = r.render_tile(&test_tile()).unwrap();
        assert!(out.contains("🧩"));
        assert!(out.contains("tile-142"));
    }

    #[test]
    fn test_telegram_status() {
        let r = TelegramRenderer::new();
        let out = r.render_status(&test_status()).unwrap();
        assert!(out.contains("🚀"));
        assert!(out.contains("executing"));
    }

    #[test]
    fn test_telegram_alert_emoji() {
        assert_eq!(TelegramRenderer::alert_emoji("green"), "🟢");
        assert_eq!(TelegramRenderer::alert_emoji("yellow"), "🟡");
        assert_eq!(TelegramRenderer::alert_emoji("red"), "🔴");
        assert_eq!(TelegramRenderer::alert_emoji("other"), "⚪");
    }

    #[test]
    fn test_telegram_gravity_emoji() {
        assert_eq!(TelegramRenderer::gravity_emoji(-1.0), "📊");
        assert_eq!(TelegramRenderer::gravity_emoji(0.0), "💬");
        assert_eq!(TelegramRenderer::gravity_emoji(1.0), "🎨");
    }

    #[test]
    fn test_telegram_ensign_role_emoji() {
        assert_eq!(TelegramRenderer::ensign_role_emoji("engine-1"), "⚙️");
        assert_eq!(TelegramRenderer::ensign_role_emoji("science-1"), "🔬");
        assert_eq!(TelegramRenderer::ensign_role_emoji("shield-1"), "🛡️");
        assert_eq!(TelegramRenderer::ensign_role_emoji("nav-1"), "📋");
        assert_eq!(TelegramRenderer::ensign_role_emoji("other"), "🤝");
    }

    #[test]
    fn test_telegram_conservation_bar() {
        let bar = TelegramRenderer::conservation_bar(750.0, 1000.0);
        assert!(bar.contains("█"));
        assert!(bar.contains("░"));
    }

    #[test]
    fn test_telegram_conservation_bar_zero() {
        let bar = TelegramRenderer::conservation_bar(0.0, 0.0);
        assert_eq!(bar, "N/A");
    }

    // ── 6. A2A Renderer ─────────────────────────────────────────────────

    #[test]
    fn test_a2a_format() {
        let r = A2ARenderer::new("test");
        assert_eq!(r.format(), RenderFormat::A2A);
    }

    #[test]
    fn test_a2a_snapshot() {
        let r = A2ARenderer::new("hermes-construct/abc123");
        let out = r.render_snapshot(&test_context()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["protocol"], "a2a/v1");
        assert_eq!(v["type"], "RepoStatus");
        assert_eq!(v["sender"], "hermes-construct/abc123");
        assert!(v["payload"].is_object());
    }

    #[test]
    fn test_a2a_room() {
        let r = A2ARenderer::new("test");
        let out = r.render_room(&test_room()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "RoomUpdate");
        assert_eq!(v["payload"]["id"], "navigation");
    }

    #[test]
    fn test_a2a_ensign() {
        let r = A2ARenderer::new("test");
        let out = r.render_ensign(&test_ensign()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "EnsignUpdate");
    }

    #[test]
    fn test_a2a_correlations() {
        let r = A2ARenderer::new("test");
        let out = r.render_correlations(&[test_correlation()]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "CorrelationDetected");
    }

    #[test]
    fn test_a2a_provenance() {
        let r = A2ARenderer::new("test");
        let out = r.render_provenance(&test_provenance()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "ProvenanceEntry");
    }

    #[test]
    fn test_a2a_tile() {
        let r = A2ARenderer::new("test");
        let out = r.render_tile(&test_tile()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "TileEvent");
    }

    #[test]
    fn test_a2a_status() {
        let r = A2ARenderer::new("test");
        let out = r.render_status(&test_status()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["type"], "RepoStatus");
    }

    // ── 7. JSON Renderer ────────────────────────────────────────────────

    #[test]
    fn test_json_format() {
        let r = JsonRenderer::new();
        assert_eq!(r.format(), RenderFormat::Json);
    }

    #[test]
    fn test_json_snapshot() {
        let r = JsonRenderer::new();
        let out = r.render_snapshot(&test_context()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["branch"], "main");
        assert_eq!(v["rooms"][0]["id"], "navigation");
    }

    #[test]
    fn test_json_room() {
        let r = JsonRenderer::new();
        let out = r.render_room(&test_room()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["id"], "navigation");
        assert_eq!(v["gravity"], -0.3);
    }

    #[test]
    fn test_json_ensign() {
        let r = JsonRenderer::new();
        let out = r.render_ensign(&test_ensign()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["model"], "Seed-2.0-mini");
    }

    #[test]
    fn test_json_correlations() {
        let r = JsonRenderer::new();
        let out = r.render_correlations(&[test_correlation()]).unwrap();
        let v: Vec<serde_json::Value> = serde_json::from_str(&out).unwrap();
        assert_eq!(v[0]["strength"], 0.87);
    }

    #[test]
    fn test_json_provenance() {
        let r = JsonRenderer::new();
        let out = r.render_provenance(&test_provenance()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["alternatives"][0]["chosen"], true);
    }

    #[test]
    fn test_json_tile() {
        let r = JsonRenderer::new();
        let out = r.render_tile(&test_tile()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["age"], 3);
    }

    #[test]
    fn test_json_status() {
        let r = JsonRenderer::new();
        let out = r.render_status(&test_status()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["rooms"], 5);
    }

    // ── 8. Markdown Renderer ────────────────────────────────────────────

    #[test]
    fn test_markdown_format() {
        let r = MarkdownRenderer::new();
        assert_eq!(r.format(), RenderFormat::Markdown);
    }

    #[test]
    fn test_markdown_snapshot() {
        let r = MarkdownRenderer::new();
        let out = r.render_snapshot(&test_context()).unwrap();
        assert!(out.contains("# Repo Snapshot"));
        assert!(out.contains("## Rooms"));
        assert!(out.contains("## Ensigns"));
        assert!(out.contains("## Correlations"));
        assert!(out.contains("## Conservation"));
    }

    #[test]
    fn test_markdown_room() {
        let r = MarkdownRenderer::new();
        let out = r.render_room(&test_room()).unwrap();
        assert!(out.contains("### Room: navigation"));
        assert!(out.contains("-0.30"));
    }

    #[test]
    fn test_markdown_ensign() {
        let r = MarkdownRenderer::new();
        let out = r.render_ensign(&test_ensign()).unwrap();
        assert!(out.contains("### Ensign: seed-mini"));
    }

    #[test]
    fn test_markdown_correlations() {
        let r = MarkdownRenderer::new();
        let out = r.render_correlations(&[test_correlation()]).unwrap();
        assert!(out.contains("engineering ↔ navigation"));
    }

    #[test]
    fn test_markdown_provenance() {
        let r = MarkdownRenderer::new();
        let out = r.render_provenance(&test_provenance()).unwrap();
        assert!(out.contains("### Provenance: p001"));
        assert!(out.contains("[✓] direct route"));
    }

    #[test]
    fn test_markdown_tile() {
        let r = MarkdownRenderer::new();
        let out = r.render_tile(&test_tile()).unwrap();
        assert!(out.contains("### Tile: tile-142"));
    }

    #[test]
    fn test_markdown_status() {
        let r = MarkdownRenderer::new();
        let out = r.render_status(&test_status()).unwrap();
        assert!(out.contains("# Status"));
        assert!(out.contains("executing"));
    }

    // ── 9. Voice Renderer ───────────────────────────────────────────────

    #[test]
    fn test_voice_format() {
        let r = VoiceRenderer::new();
        assert_eq!(r.format(), RenderFormat::Voice);
    }

    #[test]
    fn test_voice_snapshot() {
        let r = VoiceRenderer::new();
        let out = r.render_snapshot(&test_context()).unwrap();
        assert!(out.contains("executing mode"));
        assert!(out.contains("navigation room"));
        assert!(!out.contains("```"));
        assert!(!out.contains("###"));
    }

    #[test]
    fn test_voice_room() {
        let r = VoiceRenderer::new();
        let out = r.render_room(&test_room()).unwrap();
        assert!(out.contains("navigation room"));
        assert!(out.contains("yellow alert"));
    }

    #[test]
    fn test_voice_ensign() {
        let r = VoiceRenderer::new();
        let out = r.render_ensign(&test_ensign()).unwrap();
        assert!(out.contains("seed-mini"));
        assert!(out.contains("navigation room"));
    }

    #[test]
    fn test_voice_correlations() {
        let r = VoiceRenderer::new();
        let out = r.render_correlations(&[test_correlation()]).unwrap();
        assert!(out.contains("causal correlation"));
        assert!(out.contains("engineering"));
    }

    #[test]
    fn test_voice_provenance() {
        let r = VoiceRenderer::new();
        let out = r.render_provenance(&test_provenance()).unwrap();
        assert!(out.contains("chose direct route"));
        assert!(out.contains("considered scenic route"));
    }

    #[test]
    fn test_voice_tile() {
        let r = VoiceRenderer::new();
        let out = r.render_tile(&test_tile()).unwrap();
        assert!(out.contains("observation tile"));
    }

    #[test]
    fn test_voice_status() {
        let r = VoiceRenderer::new();
        let out = r.render_status(&test_status()).unwrap();
        assert!(out.contains("executing mode"));
        assert!(out.contains("3600 seconds"));
    }

    // ── 10. Render Engine ───────────────────────────────────────────────

    #[test]
    fn test_engine_new() {
        let engine = RenderEngine::new();
        assert!(engine.available_formats().is_empty());
    }

    #[test]
    fn test_engine_register() {
        let mut engine = RenderEngine::new();
        engine.register(Box::new(TerminalRenderer::new()));
        assert_eq!(engine.available_formats(), vec![RenderFormat::Terminal]);
    }

    #[test]
    fn test_engine_render() {
        let engine = default_engine();
        let ctx = test_context();
        let out = engine.render(RenderFormat::Terminal, &ctx).unwrap();
        assert!(out.contains("REPO SNAPSHOT"));
    }

    #[test]
    fn test_engine_render_room() {
        let engine = default_engine();
        let out = engine.render_room(RenderFormat::Terminal, &test_room()).unwrap();
        assert!(out.contains("navigation"));
    }

    #[test]
    fn test_engine_missing_renderer() {
        let engine = RenderEngine::new();
        let result = engine.render(RenderFormat::Terminal, &test_context());
        assert!(matches!(result, Err(RenderError::NoRendererFor(RenderFormat::Terminal))));
    }

    #[test]
    fn test_default_engine_has_all_formats() {
        let engine = default_engine();
        let formats = engine.available_formats();
        assert!(formats.contains(&RenderFormat::Terminal));
        assert!(formats.contains(&RenderFormat::Dashboard));
        assert!(formats.contains(&RenderFormat::GameEngine));
        assert!(formats.contains(&RenderFormat::Telegram));
        assert!(formats.contains(&RenderFormat::A2A));
        assert!(formats.contains(&RenderFormat::Json));
        assert!(formats.contains(&RenderFormat::Markdown));
        assert!(formats.contains(&RenderFormat::Voice));
        assert_eq!(formats.len(), 8);
    }

    // ── 11. All formats via engine ──────────────────────────────────────

    #[test]
    fn test_engine_all_formats_snapshot() {
        let engine = default_engine();
        let ctx = test_context();
        for fmt in &[
            RenderFormat::Terminal,
            RenderFormat::Dashboard,
            RenderFormat::GameEngine,
            RenderFormat::Telegram,
            RenderFormat::A2A,
            RenderFormat::Json,
            RenderFormat::Markdown,
            RenderFormat::Voice,
        ] {
            let out = engine.render(*fmt, &ctx).unwrap();
            assert!(!out.is_empty(), "format {:?} produced empty output", fmt);
        }
    }

    // ── 12. Serde roundtrip ─────────────────────────────────────────────

    #[test]
    fn test_snapshot_serde_roundtrip() {
        let snap = test_snapshot();
        let json = serde_json::to_string(&snap).unwrap();
        let back: RepoSnapshot = serde_json::from_str(&json).unwrap();
        assert_eq!(back.branch, "main");
        assert_eq!(back.rooms.len(), 2);
        assert_eq!(back.tiles.total, 230);
    }

    #[test]
    fn test_render_format_serde_roundtrip() {
        for fmt in &[
            RenderFormat::Terminal,
            RenderFormat::Dashboard,
            RenderFormat::A2A,
            RenderFormat::Voice,
        ] {
            let json = serde_json::to_string(fmt).unwrap();
            let back: RenderFormat = serde_json::from_str(&json).unwrap();
            assert_eq!(back, *fmt);
        }
    }

    #[test]
    fn test_agent_mode_serde_roundtrip() {
        for mode in &[
            AgentMode::Bootstrap,
            AgentMode::Idle,
            AgentMode::Executing,
            AgentMode::Error,
            AgentMode::Offline,
        ] {
            let json = serde_json::to_string(mode).unwrap();
            let back: AgentMode = serde_json::from_str(&json).unwrap();
            assert_eq!(back, *mode);
        }
    }

    #[test]
    fn test_render_error_serde_roundtrip() {
        let err = RenderError::SnapshotFailed("oops".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let back: RenderError = serde_json::from_str(&json).unwrap();
        assert!(matches!(back, RenderError::SnapshotFailed(_)));
    }

    #[test]
    fn test_tile_render_data_serde() {
        let tile = test_tile();
        let json = serde_json::to_string(&tile).unwrap();
        let back: TileRenderData = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, "tile-142");
    }

    #[test]
    fn test_repo_status_serde() {
        let status = test_status();
        let json = serde_json::to_string(&status).unwrap();
        let back: RepoStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(back.rooms, 5);
    }

    #[test]
    fn test_provenance_entry_serde() {
        let entry = test_provenance();
        let json = serde_json::to_string(&entry).unwrap();
        let back: ProvenanceEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(back.alternatives.len(), 2);
        assert!(back.alternatives[0].chosen);
        assert!(!back.alternatives[1].chosen);
    }

    // ── 13. Edge cases ──────────────────────────────────────────────────

    #[test]
    fn test_empty_snapshot() {
        let snap = RepoSnapshot::from_repo("/tmp/nope").unwrap();
        let ctx = RenderContext {
            repo_path: "/tmp/nope".to_string(),
            snapshot: snap,
        };
        let engine = default_engine();
        for fmt in engine.available_formats() {
            let out = engine.render(fmt, &ctx).unwrap();
            assert!(!out.is_empty());
        }
    }

    #[test]
    fn test_render_error_display() {
        assert!(RenderError::SnapshotFailed("x".into()).to_string().contains("x"));
        assert!(RenderError::NoRendererFor(RenderFormat::Terminal).to_string().contains("Terminal"));
        assert!(RenderError::RenderFailed("y".into()).to_string().contains("y"));
    }

    #[test]
    fn test_terminal_alert_colors() {
        assert_eq!(TerminalRenderer::alert_color("green"), "\x1b[32m");
        assert_eq!(TerminalRenderer::alert_color("yellow"), "\x1b[33m");
        assert_eq!(TerminalRenderer::alert_color("red"), "\x1b[31m");
        assert_eq!(TerminalRenderer::alert_color("other"), "\x1b[0m");
    }

    #[test]
    fn test_default_implementations() {
        let _ = TerminalRenderer::default();
        let _ = DashboardRenderer::default();
        let _ = GameEngineRenderer::default();
        let _ = TelegramRenderer::default();
        let _ = JsonRenderer::default();
        let _ = MarkdownRenderer::default();
        let _ = VoiceRenderer::default();
        let _ = RenderEngine::default();
    }
}
