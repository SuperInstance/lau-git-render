# lau-git-render

**THE rendering layer.** Same agent state, 8 output formats. One render call.

```rust
use lau_git_render::*;

let engine = default_engine();  // all 8 renderers registered
let ctx = RenderContext::from_repo("/tmp/my-agent")?;

// One call, any format
let terminal = engine.render(RenderFormat::Terminal, &ctx)?;
let telegram = engine.render(RenderFormat::Telegram, &ctx)?;
let scene    = engine.render(RenderFormat::GameEngine, &ctx)?;
```

## 8 Renderers, Same State

| Format | Consumer | Output |
|--------|----------|--------|
| **Terminal** | CLI users | ASCII art with box-drawing + ANSI color |
| **Dashboard** | Web UIs | JSON widget array (room cards, gauges, charts) |
| **GameEngine** | Unity/Godot | Scene graph with 3D positions, connections, lighting |
| **Telegram** | Chat bots | Emoji markdown with conservation progress bar |
| **A2A** | Other agents | Structured protocol messages (type, sender, payload) |
| **JSON** | APIs | Raw JSON dump |
| **Markdown** | Docs | Readable markdown with headers and sections |
| **Voice** | TTS | Plain conversational text, no special characters |

## Output Examples

### Terminal (ASCII Art)
```
╔══════════════════════════════════════╗
║          ROOM LAYOUT GRID           ║
╠══════════════════════════════════════╣
║ ┌─navigation──────┐ ║
║ │ gravity:  -0.30  │ ║
║ │ alert: yellow    │ ║
║ │ tiles: 142       │ ║
║ │  ↳ seed-mini (Seed-2.0-mini) ║
║ └──────────────────┘ ║
╚══════════════════════════════════════╝
```

### Dashboard (JSON Widgets)
```json
{
  "widgets": [
    {"type": "room", "id": "navigation", "gravity": -0.3, "alert": "yellow"},
    {"type": "gauge", "id": "conservation", "value": 7500.0, "max": 10000.0}
  ],
  "layout": "grid"
}
```

### GameEngine (Scene Graph)
```json
{
  "entities": [
    {"id": "room_navigation", "type": "room", "position": [0, 0, 0]},
    {"id": "ensign_seed-mini", "type": "ensign", "parent": "room_navigation"}
  ],
  "environment": {"lighting": "normal"}
}
```

### Telegram (Emoji Markdown)
```
🚀 **Repo Status** — `main`
🟡 💬 `navigation` — gravity -0.30, 142 tiles yellow
💰 Conservation: [███████░░░] 7500/10000
```

## The One-Line Render Call

```rust
let output = default_engine().render(RenderFormat::Telegram, &ctx)?;
```

That's it. Register custom renderers, override formats, or use the built-in 8.

## Custom Renderers

```rust
struct MyRenderer;
impl Renderer for MyRenderer {
    fn format(&self) -> RenderFormat { RenderFormat::Terminal }
    // ... implement 7 render methods
}

let mut engine = RenderEngine::new();
engine.register(Box::new(MyRenderer));
```

## Tests

**96 tests** — every renderer × every method, serde roundtrips, edge cases (empty snapshots), error paths, engine routing, all 8 formats via engine.

## Ecosystem

- [lau-shell-kernel] — bare construct
- [lau-provider] — LLM provider abstraction
- [lau-tile-store] — SQLite-backed tile persistence
- [lau-git-agent] — repo-as-agent
- **[lau-git-render]** (this) — multi-format rendering

## License

MIT
