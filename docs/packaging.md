# 📦 Packaging Systems

## 🎯 Goal

Unify multiple package systems without hiding differences.

---

## 📚 Supported Formats

### Flatpak
- Sandboxed
- User-level installs
- Good isolation

---

### Snap (Optional)
- Strong sandboxing
- Requires snapd
- Can be disabled

---

### AppImage
- Portable binaries
- No install required
- Limited integration

---

### Native Packages

#### APT
- Debian/Ubuntu

#### DNF
- Fedora

#### Pacman
- Arch

---

## ⚖️ Comparison Factors

- Security
- Performance
- Disk usage
- Update model
- System integration

---

## 🔄 Format Switching

Users can:
- choose format manually
- switch between formats
- compare before installing

---

## ⚠️ Challenge

Different systems:
- use different dependency models
- have different versioning
- behave differently

---

## 🧠 Rule

> Differences must be exposed, not hidden.
