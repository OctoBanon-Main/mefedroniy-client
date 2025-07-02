---
name: "\U0001F31F Client Suggestion"
about: Suggest a new RAC client identifier so that it can be recognized and styled
  in Mefedroniy Client
title: ''
labels: user agents
assignees: OctoBanon-Main

---

### 🧾 Client Name

<!-- Display name of the client (as it should appear in UI) -->
Example: `Nebula`, `XenRAC`, `RACforWin`

### 🔣 User Agent Symbol

<!-- Unique Unicode prefix used in messages -->
Example: `☄` or Unicode escape: `\u2604`

### 🧬 Regular Expression

<!-- Regex to extract username and message.
Must have two capture groups: (1) username, (2) message. -->

```regex
\u2604<(.*?)> (.*)
```

### 🎨 Preferred Color

<!-- Suggested color for the username when this agent is detected -->
Example: Cyan, Light Blue, Gold, #FF9900

### ✅ Checklist

- [ ] The symbol and color are unique (not used by another known client).
- [ ] The regex correctly matches messages sent by this client.
