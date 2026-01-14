# VCT v1 (Legacy Prototype)

> ⚠️ **WARNING: Experimental & Unsafe**  
> This is **VCT v1**, the very first prototype of VCT, created purely as a **learning experiment** to understand file system operations in **Rust**.

This version is **poorly written**, **not production-ready**, and **can be dangerous if misused**.  
In particular, the `goto` command can **wipe out large portions of your filesystem** if run in the wrong directory.

**Linux users: be extremely careful.**

---

## What is VCT?

VCT (Version Control Tool) is an **internal, proprietary version control system** developed by **STUPA STUDIO**.

This repository showcases **only the very first public prototype (v1)** — a minimal, naïve implementation built while learning Rust’s filesystem APIs.

Modern VCT has evolved far beyond this.

---

## ⚠️ Important Disclaimer

- ❌ **Do NOT use this on important projects**
- ❌ **Do NOT run `goto` outside a test directory**
- ❌ **Do NOT treat this as a Git alternative**
- ✔️ This exists **only for educational and historical purposes**

You have been warned.

---

## Features in VCT v1

- Initialize a project directory
- Store full snapshots of the filesystem
- Generate commits using SHA-1 hashing
- Restore files from a commit (`goto`)
- Simple commit history log

No branches.  
No safety checks.  
No permissions.  
No mercy.

---

## Commands

### Initialize a Project

```bash
vct make <project_name>
```

Creates a new project directory and initializes the `.vct` structure.

---

### Store a Commit

```bash
vct store -m "commit message"
```

Stores a full snapshot of the current directory as a commit.

---

### Show Commit History

```bash
vct show
```

Displays all stored commits.

---

### Restore a Commit (DANGEROUS)

```bash
vct goto <commit_id>
```

⚠️ This command deletes all files recursively except `.vct` and `vct.exe`.  
Running this in the wrong directory **can wipe your system**.

---

## Why This Exists

VCT v1 was built to:

- Learn Rust filesystem APIs (`std::fs`)
- Experiment with recursive file traversal
- Understand hashing and snapshot-based versioning
- Explore how version control systems work internally

This version was never meant to be safe, clean, or public.

---

## Modern VCT

VCT is currently in **v4**, featuring:

- Server-based commits
- Branching
- Authentication
- Encryption
- Internal tooling integrations

**VCT v4 is proprietary software** and is used exclusively as an **internal pipeline tool** at **STUPA STUDIO**.  
Its source code cannot be shared.

---

## Downloads

- **VCT v1 (Windows EXE)** is available at:  
  https://rijankoirala.com.np

This release is shared purely as a **historical snapshot** of early development.

---

## Final Notes

This repository is **not a product**.  
It is **not supported**.  
It is **not safe**.

It exists only to document the early evolution of VCT.

— **Rizz / STUPA STUDIO**
