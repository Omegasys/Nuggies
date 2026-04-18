# 🏗️ Nuggies Architecture

## 🎯 Overview

Nuggies is a **modular, CLI-first system** that acts as a transparent abstraction layer over Linux package managers.

It does not replace package managers.
It orchestrates and exposes them.

---

## 🧩 High-Level Components
User (CLI / TUI / GUI)
↓
CLI Layer
↓
Core Layer
↓
Package Abstraction (pkg/)
↓
Backend Adapters (APT, Flatpak, etc.)
↓
System Package Managers


---

## 📦 Core Modules

### CLI (`cli/`)
- Parses commands
- Formats output
- Delegates logic to core

### Core (`core/`)
- Shared utilities
- Config handling
- Logging
- Error management

### Package Layer (`pkg/`)
- Unified package model
- Backend selection
- Dependency handling

### Transparency (`transparency/`)
- Command previews
- Dependency trees
- Permission inspection

### Security (`security/`)
- Signature verification
- Trust scoring
- Sandbox auditing

### Automation (`automation/`)
- Parses `nuggies.yaml`
- Executes batch operations

---

## 🔌 Backend Adapter Pattern

Each package system is implemented as a backend:

- flatpak.rs
- apt.rs
- pacman.rs
- etc.

Each backend must implement:

- search()
- install()
- remove()
- update()
- info()

---

## 🔄 Data Flow Example

Install flow:

1. CLI parses: `nuggies install firefox`
2. pkg layer resolves:
   - available formats
   - user preferences
3. transparency generates preview
4. user confirms (optional)
5. backend executes command
6. result returned and logged

---

## ⚠️ Key Challenges

- Normalizing package metadata across systems
- Handling different dependency models
- Maintaining transparency without overwhelming users

---

## 🧠 Design Rule

> No module should hide system behavior.
