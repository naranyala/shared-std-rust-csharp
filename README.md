# 🖥️ Shared Desktop StdLib (Rust & C#)

A high-performance, cross-language standard library designed for desktop applications. This project provides a set of common utility modules implemented in **Rust** for safety and speed, exposed via a C-compatible FFI (Foreign Function Interface) for seamless consumption in **C#/.NET**.

## 🏗️ Architecture

The library is compiled as a `cdylib` (C-compatible dynamic library). It follows a "Gateway" pattern:
- **Internal Logic**: Pure, idiomatic Rust modules.
- **FFI Gateway**: A flat C-API layer that handles pointer marshalling and memory management.
- **C# Wrapper**: P/Invoke signatures that allow .NET to call Rust functions as if they were native.

## 🚀 Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (Stable)
- [.NET 10.0 SDK](https://dotnet.microsoft.com/download)

### 1. Build the Library
```bash
cargo build --release
```
The binary will be located at `target/release/libshared_std_rust_csharp.so` (Linux), `.dll` (Windows), or `.dylib` (macOS).

### 2. Run the Demo
```bash
# Set the library path for the .NET runtime (Linux example)
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(pwd)/target/debug
cd csharp_example
dotnet run
```

---

## 📚 API Reference

### 🛠️ 1. System & Environment (`SysInfo`)
Provides basic information about the host machine.

| Function | C# Signature | Description |
| :--- | :--- | :--- |
| `desktop_get_os` | `IntPtr desktop_get_os()` | Returns the OS name (e.g., "linux", "windows"). |
| `desktop_get_arch` | `IntPtr desktop_get_arch()` | Returns the CPU architecture (e.g., "x86_64"). |

### 📝 2. Logging System (`Logging`)
A thread-safe logger that allows C# to leverage Rust's synchronization.

| Function | C# Signature | Description |
| :--- | :--- | :--- |
| `desktop_log` | `void desktop_log(int level, string msg)` | Logs a message. Levels: `0: Info`, `1: Warn`, `2: Error`. |

### 📁 3. File System Utilities (`FS Utils`)
Desktop-centric file and directory operations.

| Function | C# Signature | Description |
| :--- | :--- | :--- |
| `desktop_ensure_dir` | `bool desktop_ensure_dir(string path)` | Checks if a directory exists; creates it if it doesn't. |
| `desktop_get_ext` | `IntPtr desktop_get_ext(string path)` | Extracts the file extension from a path. |

### ⚙️ 4. Configuration Manager (`Config`)
A stateful JSON-based settings manager. It uses **Opaque Pointers** to maintain state in Rust.

| Function | C# Signature | Description |
| :--- | :--- | :--- |
| `config_load` | `IntPtr config_load(string path)` | Loads a JSON config from disk into Rust memory. |
| `config_set` | `void config_set(IntPtr cfg, string k, string v)` | Updates a setting in the loaded config. |
| `config_get` | `IntPtr config_get(IntPtr cfg, string k)` | Retrieves a setting value. |
| `config_save` | `bool config_save(IntPtr cfg, string path)` | Persists the current config to a JSON file. |
| `config_destroy` | `void config_destroy(IntPtr cfg)` | Frees the Rust-allocated config object. |

### 🔢 5. Core Utilities (`Math` & `Text`)
Stateless helper functions.

| Function | C# Signature | Description |
| :--- | :--- | :--- |
| `math_add` | `int math_add(int a, int b)` | Adds two integers. |
| `math_multiply` | `int math_multiply(int a, int b)` | Multiplies two integers. |
| `text_to_uppercase` | `IntPtr text_to_uppercase(string s)` | Converts string to uppercase. |
| `text_reverse` | `IntPtr text_reverse(string s)` | Reverses the characters in a string. |

### 👤 6. State Management (`Session`)
Demonstrates how to manage complex Rust structs from C#.

| Function | C# Signature | Description |
| :--- | :--- | :--- |
| `session_create` | `IntPtr session_create(uint id, string name)` | Creates a new user session. |
| `session_add_score` | `void session_add_score(IntPtr s, int pts)` | Modifies state within the session. |
| `session_get_score` | `int session_get_score(IntPtr s)` | Reads state from the session. |
| `session_destroy` | `void session_destroy(IntPtr s)` | Frees the session memory. |

---

## ⚠️ Critical: Memory Management

Because Rust and C# use different memory management systems (Manual/Ownership vs. Garbage Collection), you must follow these rules to avoid memory leaks:

### 1. String Handling
Any function returning an `IntPtr` that represents a string (like `desktop_get_os` or `text_reverse`) allocates memory on the Rust heap. You **must** free it using:
```csharp
IntPtr ptr = text_reverse("Hello");
string result = Marshal.PtrToStringAnsi(ptr);
free_string(ptr); // <--- REQUIRED
```

### 2. Stateful Objects (Handles)
Functions like `config_load` and `session_create` return a pointer to a Rust object. C# treats this as an opaque handle. You must call the corresponding destroy function when finished:
```csharp
IntPtr cfg = config_load("settings.json");
// ... use config ...
config_destroy(cfg); // <--- REQUIRED
```

## 🛠️ Extending the Library

To add a new module:
1. Create a new file in `src/modules/my_module.rs`.
2. Implement your logic using idiomatic Rust.
3. Declare the module in `src/lib.rs` using `mod modules { pub mod my_module; }`.
4. Create `#[unsafe(no_mangle)] pub extern "C"` wrapper functions in `src/lib.rs`.
5. Add the `[DllImport]` signature in C#.
