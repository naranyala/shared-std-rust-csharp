using System;
using System.Runtime.InteropServices;

class DesktopApp
{
    const string LibName = "libshared_std_rust_csharp";

    // --- Desktop StdLib Imports ---
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void desktop_log(int level, string message);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr desktop_get_os();

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr desktop_get_arch();

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern bool desktop_ensure_dir(string path);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr desktop_get_ext(string path);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr config_load(string path);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void config_set(IntPtr cfg, string key, string val);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr config_get(IntPtr cfg, string key);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern bool config_save(IntPtr cfg, string path);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void config_destroy(IntPtr cfg);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_string(IntPtr ptr);

    // --- New StdLib Imports ---
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr crypto_sha256(string input);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr crypto_encode_base64(string input);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern bool shell_open_url(string url);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr net_http_get(string url);

    static void Main()
    {
        Console.WriteLine("=== Desktop Shared StdLib Demo ===\n");

        // 1. System Info
        IntPtr osPtr = desktop_get_os();
        IntPtr archPtr = desktop_get_arch();
        Console.WriteLine($"OS: {Marshal.PtrToStringAnsi(osPtr)}");
        Console.WriteLine($"Arch: {Marshal.PtrToStringAnsi(archPtr)}");
        free_string(osPtr);
        free_string(archPtr);

        // 2. Logging
        desktop_log(0, "Application started successfully.");
        desktop_log(1, "Warning: Resource usage is high.");
        desktop_log(2, "Error: Failed to connect to server.");

        // 3. File System
        string myDir = "./app_data";
        if (desktop_ensure_dir(myDir)) {
            Console.WriteLine($"Directory {myDir} is ready.");
        }
        
        IntPtr extPtr = desktop_get_ext("test.config.json");
        Console.WriteLine($"Extension of test.config.json: {Marshal.PtrToStringAnsi(extPtr)}");
        free_string(extPtr);

        // 4. Configuration
        string configPath = "./settings.json";
        IntPtr cfg = config_load(configPath);
        config_set(cfg, "theme", "dark");
        IntPtr valPtr = config_get(cfg, "theme");
        Console.WriteLine($"Config Theme: {Marshal.PtrToStringAnsi(valPtr)}");
        free_string(valPtr);
        config_save(cfg, configPath);
        config_destroy(cfg);

        // 5. Crypto
        IntPtr hashPtr = crypto_sha256("SecretPassword123");
        Console.WriteLine($"SHA256 Hash: {Marshal.PtrToStringAnsi(hashPtr)}");
        free_string(hashPtr);

        IntPtr b64Ptr = crypto_encode_base64("Hello Rust!");
        Console.WriteLine($"Base64: {Marshal.PtrToStringAnsi(b64Ptr)}");
        free_string(b64Ptr);

        // 6. Networking
        Console.WriteLine("Fetching google.com (first 100 chars)...");
        IntPtr netPtr = net_http_get("https://www.google.com");
        string netRes = Marshal.PtrToStringAnsi(netPtr);
        Console.WriteLine(netRes.Substring(0, Math.Min(100, netRes.Length)));
        free_string(netPtr);

        // 7. Shell
        Console.WriteLine("Opening browser to GitHub...");
        shell_open_url("https://github.com");

        Console.WriteLine("\nDemo finished.");
    }
}
