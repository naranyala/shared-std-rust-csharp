# 🚀 Shared Desktop StdLib (Rust & C#)

A professional-grade, cross-language standard library for high-performance desktop applications. This project enables C# developers to offload compute-heavy, system-level, or security-critical tasks to **Rust** while maintaining a seamless .NET developer experience.

## 💎 The "Best of Both Worlds" Architecture

This library isn't just a set of utilities; it's designed to leverage the specific strengths of each ecosystem:

### 🦀 Rust $\rightarrow$ C# (Power & Safety)
- **Fearless Concurrency**: Uses `Rayon` for data-parallel processing across all CPU cores without data races.
- **Linear-Time Regex**: Leverages Rust's `regex` crate to avoid catastrophic backtracking.
- **Memory-Safe Crypto**: Implements SHA256 and Base64 using Rust's strict type system.
- **Low-Level OS Integration**: Direct access to system APIs and efficient file I/O.

### 🔷 C# $\rightarrow$ Rust (Flexibility & UI)
- **Event Bridge**: Implements a Reverse FFI system where Rust can trigger C# delegates, enabling Rust background tasks to update C# UIs in real-time.
- **Rapid Prototyping**: High-level application flow and UI managed in .NET.

---

## 📚 API Reference

### ⚡ High-Performance Modules
| Function | Signature | Description |
| :--- | :--- | :--- |
| `parallel_sum` | `double(IntPtr data, int len)` | Parallel reduction sum of a double array. |
| `parallel_square` | `void(IntPtr data, int len)` | Parallel mutation of array elements. |
| `regex_is_match` | `bool(string pat, string txt)` | High-speed regex matching. |
| `regex_replace` | `IntPtr(string pat, string txt, string repl)` | High-speed global string replacement. |

### 🔔 Event Bridge (Reverse FFI)
| Function | Signature | Description |
| :--- | :--- | :--- |
| `register_event_callback` | `void(IntPtr callback)` | Registers a C# delegate to be called by Rust. |
| `trigger_rust_event` | `void(string message)` | Triggers the registered C# callback. |

### 🛡️ Security & Networking
| Function | Signature | Description |
| :--- | :--- | :--- |
| `crypto_sha256` | `IntPtr(string input)` | Computes a SHA256 hex hash. |
| `crypto_encode_base64` | `IntPtr(string input)` | Base64 encodes a string. |
| `net_http_get` | `IntPtr(string url)` | Synchronous HTTP GET request. |
| `shell_open_url` | `bool(string url)` | Opens a URL in the default system browser. |

### ⚙️ Desktop Utilities
| Module | Capabilities |
| :--- | :--- |
| **Config** | Stateful JSON settings (`config_load`, `config_save`, `config_set`). |
| **FS Utils** | Folder creation, extension extraction (`desktop_ensure_dir`, `desktop_get_ext`). |
| **SysInfo** | OS and Architecture detection (`desktop_get_os`, `desktop_get_arch`). |
| **Logging** | Thread-safe system-wide logging (`desktop_log`). |

---

## 🛠️ Getting Started

### 1. Build
```bash
cargo build --release
```

### 2. C# Integration
Copy `libshared_std_rust_csharp.so` (or `.dll`) to your execution folder and use the `DllImport` signatures provided in the `csharp_example` project.

## ⚠️ Memory Safety Rules

1. **Strings**: Any `IntPtr` returned as a string **must** be freed with `free_string(ptr)`.
2. **Handles**: Any `IntPtr` representing a stateful object (e.g., `AppConfig`, `UserSession`) **must** be destroyed using the corresponding `destroy` function.
3. **Callbacks**: When passing a delegate to Rust, ensure the delegate is pinned or stored in a static variable to prevent the .NET Garbage Collector from moving it.
