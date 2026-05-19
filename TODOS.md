# 🗺️ Project Roadmap (TODOS)

This document outlines the future evolution of the Shared Desktop StdLib.

## 🔴 High Priority (Core Infrastructure)
- [ ] **Async Bridge**: Implement `async/await` support between Rust (Tokio) and C# (Task). This is the most requested feature for networking.
- [ ] **Strongly Typed Structs**: Move away from `IntPtr` for data exchange and implement `C-compatible structs` (using `#[repr(C)]`) for complex data passing.
- [ ] **Automated CI/CD**: Set up GitHub Actions to build and test the library for Windows, Linux, and macOS on every push.

## 🟡 Medium Priority (Feature Expansion)
- [ ] **Database Module**: Add a high-performance SQLite wrapper in Rust to provide C# with an optimized local data store.
- [ ] **Graphics/Image Processing**: Integrate `image-rs` to provide C# with fast image resizing, filtering, and format conversion.
- [ ] **Process Management**: Implement a more robust `Shell` module capable of managing child processes and capturing real-time stdout/stderr streams.
- [ ] **Hardware Info**: Expand `SysInfo` to include CPU usage, RAM availability, and GPU detection.

## 🟢 Low Priority (Optimization & Polish)
- [ ] **Benchmarking Suite**: Create a comparison suite to prove the performance gains of `parallel_sum` vs C# `Parallel.ForEach`.
- [ ] **NuGet Packaging**: Create a NuGet package that bundles the Rust binaries for different platforms.
- [ ] **Advanced Logging**: Implement log rotation and file-based logging in the `Logging` module.
- [ ], **JSON Schema Validation**: Add schema validation to the `Config` module to prevent crashes on malformed settings files.
