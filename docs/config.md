# ⚙️ Configuration

## 📍 Location

Default:
~/.config/nuggies/


---

## 📄 Format

Supported:
- TOML (preferred)
- YAML (optional)

---

## 🧩 Example Config

```toml
default_format = "auto"
enable_snap = false
offline_mode = false
strict_security = true
max_parallel = 4
🔄 Priority Order
CLI flags
Environment variables
Config file
Built-in defaults
🔐 Philosophy
No hidden config
Everything user-editable
Explicit over implicit
📦 Config Categories
General
format preferences
concurrency
Security
trust level
sandbox strictness
Paths
cache
config
logs
⚠️ Rule

If a setting changes behavior, it must be visible and documented.


---

# 📁 `docs/security.md`

```markdown
# 🔐 Security Model

## 🎯 Goals

- Transparency over blind trust
- User awareness over automation
- No hidden security decisions

---

## 🔑 Core Principles

- Zero telemetry
- Explicit trust decisions
- Inspectable operations

---

## 🧩 Components

### Signature Verification
- Uses system tools where possible
- Displays verification status

### Trust Scoring
Factors:
- signature validity
- source reputation
- reproducibility

---

### Sandbox Inspection

Shows:
- filesystem access
- network access
- device permissions

---

### Permission Alerts

Triggered when:
- new permissions appear
- sandbox changes

---

## 🌐 Network Awareness

Where possible:
- detect network usage
- warn user

---

## ⚠️ Limitations

- Cannot enforce sandboxing universally
- Relies on backend capabilities

---

## 🧠 Philosophy

> Nuggies does not decide what is safe.  
> It shows you what is happening.
