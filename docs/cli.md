# 🖥️ Nuggies CLI

## 🎯 Design Goals

- Predictable
- Scriptable
- Transparent
- Minimal abstraction

---

## 📌 Command Structure
nuggies <command> [options] [arguments]


---

## 🔧 Core Commands

### Search

nuggies search <query>


### Install

nuggies install <package>


Options:
- `--format=<type>`
- `--dry-run`
- `--json`

---

### Remove

nuggies remove <package>


---

### Update

nuggies update [package]


---

### Compare

nuggies compare <package>


Shows:
- formats available
- size
- sandboxing
- trust level

---

### Rollback

nuggies rollback <package> <version>


---

### Explain

nuggies explain <package>


---

### Doctor

nuggies doctor


---

## 📤 Output Modes

- Default (human-readable)
- JSON (`--json`)
- Plain text (`--plain`)

---

## 🧠 Design Rules

- No hidden side effects
- All commands should support dry-run
- Output must be script-friendly
