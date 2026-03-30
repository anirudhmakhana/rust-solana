# CLAUDE.md

## After Every Task

After solving any coding task or exercise:

1. **Create a new file** in `src/NAME.rs` — each exercise gets its own standalone binary
2. **Register it** in `Cargo.toml` as a `[[bin]]` entry
3. **Append a summary** to `notes.md` — follow the W2-PXX numbering and style below

### File conventions (`src/NAME.rs`)

- Each file is a complete standalone binary with its own `fn main()`
- Every non-obvious line gets a `// NOTE:` comment explaining the *why*
- First two lines: a comment naming the topic, and `// Run: cargo run --bin NAME`
- No interdependencies between files — each compiles on its own

### Notes conventions (`notes.md`)

- Heading: `### W2-PXX — Topic Name (\`src/NAME.rs\`)`
- Link to official Rust docs
- One-line description of what the concept is and why it matters
- Concise code block with `// NOTE:` comments
- `#### Key ideas` bullet list — explain *why*, not just *what*

Example:

```markdown
### W2-PXX — Topic Name (`src/NAME.rs`)

**[Docs: ...](https://doc.rust-lang.org/...)**

One-line description.

```rust
// NOTE: explain the key mechanic here
code_example();
```

#### Key ideas

- bullet explaining why, not just what
```
