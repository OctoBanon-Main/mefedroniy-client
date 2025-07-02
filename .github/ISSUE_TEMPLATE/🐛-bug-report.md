---
name: "\U0001F41B Bug Report"
about: Create a report to help us improve
title: ''
labels: bug
assignees: OctoBanon-Main

---

**What happened?**
<!-- One-liner. Example: "Panic when entering special characters in input field" -->

---

### 🔁 Reproduction Steps

1. Run: `RUST_BACKTRACE=1 ./mefedroniy-client-vX.Y.Z+A.B.C-linux-x86_64`
2. Perform: [describe the exact actions]
3. Error occurs when: [e.g., submitting the form]

---

### ✅ Expected Behavior

<!-- Describe what *should* have happened. Example: "Program should handle input gracefully without crashing." -->

---

### ❌ Actual Behavior

Check all that apply:
- [  ] Rust panic (full message below)
- [  ] Incorrect output
- [  ] UI/Visual bug
- [  ] Performance degradation
- [  ] Other: [describe]

---

### 💥 Crash Details (if applicable)

```rust
// Full panic message with backtrace
thread 'main' panicked at '...', src/main.rs:123:45
stack backtrace:
   0: ...
```

---

### 🖥 System Information

Client version: mefedroniy-client-vX.Y.Z+A.B.C-[os]-[arch]
OS: [Windows/macOS/Linux + version]
Terminal: [eg. "Windows Terminal", "Alacritty", "GNOME Terminal"]
Window size: [eg. 80x24]

---

### 📸 UI Issues (if applicable)

 - [ ] Screenshot includes entire window
 - [ ] Screenshot uses original colors
 - [ ] Screenshot shows error context clearly

---

### 🔍 Diagnostics

- Frequency: [Every time / Sometimes / Rare]
- Regression? [Did it work in a previous version? If so, which one?]
- Temporary workaround: [If found]

### 🧹 Pre-Submission Checklist

---

- [ ] Reproduced with RUST_BACKTRACE=full
- [ ] Tested latest version
