# Habit - a tiny CLI habit tracker (Rust)


Track daily/weekly habits from your terminal. Fast, local-first, and human-readable (JSON store).

---

## Code **Example**

---

`$ habit add "Read fresh blogs for 30 mins" --goal daily --tags mind, refreshing`
`$ habit done "Read fresh blogs for 30 mins"`
`$ habit list`

- Meditate 10m [daily] tags: mind, refreshing
  
`$ habit streak "Read fresh blogs for 30 mins"`

- Streak — current: 1 | best: 1

`$ habit stats --global`

Global Stats

- Active habits: 1
- Completions this week: 1
- Completions this month: 1

---

## ✨ Features (MVP)

---

- Add/list habits with daily, weekly, or every:N schedules
- Mark done/undo for any date (default: today)
- Streaks (current & best) for daily/weekly goals
- Global stats (week/month completions, active habits)
- Safe JSON storage with atomic writes (portable & easy to back up)

> Storage is local by default using platform-specific data dirs.
>  
> Run `habit init` to see the exact file path on   your machine.

---

## 🔧 Install

### Build from source
```bash
# 1) Prereqs: Rust toolchain
# https://rustup.rs

# 2) Clone and build
git clone https://github.com/<you>/habit.git
cd habit

# 3) Build release & install
cargo build --release
cargo install --path .

If habit isn’t found after install, add Cargo’s bin dir to your PATH (e.g., zsh):

echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

🚀 Quickstart

habit init
habit add "Read 20 pages" --goal daily --tags learning,reading
habit done "Read 20 pages"
habit list
habit streak "Read 20 pages"
habit stats --global
```

## 🧰 Commands

| Command                                                           | What it does                                 | Examples                                            |
| ----------------------------------------------------------------- | -------------------------------------------- | --------------------------------------------------- |
| `init`                                                            | Create config/data directories & empty store | `habit init`                                        |
| `add <name> [--goal <daily\|weekly\|every:N>] [--tags tag1,tag2]` | Add a habit                                  | `habit add "Gym" --goal weekly --tags health`       |
| `list [--all] [--archived] [--tag <tag>]`                         | List habits                                  | `habit list`, `habit list --tag health`             |
| `done <name> [--date YYYY-MM-DD]`                                 | Mark a habit as done (default: today)        | `habit done "Gym" --date 2025-09-01`                |
| `undo <name> [--date YYYY-MM-DD]`                                 | Remove a completion                          | `habit undo "Gym"`                                  |
| `streak <name>`                                                   | Show current/best streak for a habit         | `habit streak "Gym"`                                |
| `stats [--global \| --habit <name>]`                              | Stats overview                               | `habit stats --global`, `habit stats --habit "Gym"` |

> Frequency grammar: `daily`, `weekly`, or `every:N` (e.g., every:3 for every 3 days).

## 🗂️ Data & Config

- Uses platform data dirs via [directories](https://crates.io/crates/directories?utm_source=chatgpt.com).
- Data file is JSON. Schema (v1):

``` json
{
  "schema_version": 1,
  "habits": [
    {
      "id": "uuid",
      "name": "Meditate 10m",
      "frequency": { "type": "Daily" },
      "tags": ["health","mindfulness"],
      "created_at": "2025-09-03T10:35:00Z",
      "archived": false
    }
  ],
  "completions": [
    {
      "habit_id": "uuid",
      "date": "2025-09-03",
      "created_at": "2025-09-03T10:36:00Z"
    }
  ]
}
```

- **Atomic writes**: updates write to a temp file then rename.
- **Timezones**: completions store date-only to avoid DST/offset churn; “today” uses your system local date

## 🧪 Development

``` bash
# Format & lint
cargo fmt
cargo clippy -- -D warnings

# Tests
cargo test

# Run locally (without installing)
cargo run -- init
cargo run -- add "Meditate 10m" --goal daily --tags health
cargo run -- done "Meditate 10m"
cargo run -- list

```

### Folder Structure

``` csharp
src/
├─ main.rs          # entry
├─ cli.rs           # clap CLI definitions
├─ model.rs         # Habit, Completion, Frequency, DataFile
├─ store.rs         # JSON store (atomic read/write, filtering)
├─ util.rs          # dates, helpers
└─ commands/        # subcommand handlers
   ├─ init.rs
   ├─ add.rs
   ├─ list.rs
   ├─ done.rs
   ├─ streak.rs
   └─ stats.rs

```

Key Crates:

- CLI: `clap`
- Data: `serde`, `serde_json`, `uuid`
- Time: `chrono`
- Paths: `directories`
- Errors: `anyhow`, `thiserror`
- Sync: `parking_lot`

## 🤝 Contributing

PRs welcome! Please:

1. Open an issue describing the change.
2. Run ``cargo fmt``, ``cargo clippy``, and ``cargo test``.
3. Keep user-facing messages short and clear.

## 📝 License

MIT © Shahin Yahya
