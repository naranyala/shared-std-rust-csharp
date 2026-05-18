# Shared Rust-C# Library

This project demonstrates how to create a shared library in Rust that can be consumed by both other Rust projects and C# applications. It leverages Rust's Foreign Function Interface (FFI) and `cdylib` crate type to provide a C-compatible binary.

## 🚀 Features

- **Cross-Language Compatibility**: Compiled as both `rlib` (for Rust) and `cdylib` (for C# / C / C++).
- **Type Mapping**: Demonstrates passing simple integers and complex types like strings across the FFI boundary.
- **Memory Safety**: Provides a dedicated memory cleanup function to prevent leaks when passing heap-allocated data from Rust to C#.

## 📁 Project Structure

```text
.
├── Cargo.toml            # Rust package configuration
├── src
│   └── lib.rs            # Rust library implementation (FFI exports)
└── csharp_example        # C# project to consume the Rust library
    ├── Example.csproj
    └── Program.cs
```

## 🛠️ Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (including `cargo`)
- [.NET 8.0 SDK](https://dotnet.microsoft.com/download)

### 1. Building the Rust Library

Build the project in debug or release mode:

```bash
# Build for development
cargo build

# Build for production (optimized)
cargo build --release
```

This produces a shared library file:
- **Linux**: `target/debug/libshared_std_rust_csharp.so`
- **Windows**: `target/debug/shared_std_rust_csharp.dll`
- **macOS**: `target/debug/libshared_std_rust_csharp.dylib`

### 2. Consuming in C#

The C# example uses `DllImport` (P/Invoke) to call the Rust functions.

1. Navigate to the example folder:
   ```bash
   cd csharp_example
   ```

2. Set the library path so the .NET runtime can find the `.so`/`.dll` file:
   - **Linux**:
     ```bash
     export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:../target/debug
     ```
   - **Windows**: Copy the `.dll` file from `target/debug/` into the `csharp_example/` folder.

3. Run the application:
   ```bash
   dotnet run
   ```

### 3. Consuming in Rust

Since the library is also an `rlib`, you can add it to another Rust project as a dependency:

```toml
[dependencies]
shared-std-rust-csharp = { path = "../path/to/this/project" }
```

## ⚠️ Important: Memory Management

When Rust returns a string to C#, it returns a pointer to memory allocated on the Rust heap. **C#'s Garbage Collector cannot manage this memory.**

To prevent memory leaks, you must call the provided `free_string` function once you are done with the string in C#:

```csharp
IntPtr ptr = hello_rust("Name");
try {
    string result = Marshal.PtrToStringAnsi(ptr);
    Console.WriteLine(result);
} finally {
    free_string(ptr); // Crucial: Free Rust-allocated memory
}
```

## 🛠️ API Reference

| Function | Rust Signature | C# P/Invoke Signature | Description |
| :--- | :--- | :--- | :--- |
| `add_numbers` | `fn(i32, i32) -> i32` | `int add_numbers(int a, int b)` | Adds two integers. |
| `hello_rust` | `fn(*const c_char) -> *mut c_char` | `IntPtr hello_rust(string name)` | Returns a greeting string. |
| `free_string` | `fn(*mut c_char)` | `void free_string(IntPtr ptr)` | Frees memory allocated by `hello_rust`. |
