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
        config_set(cfg, "window_width", "1280");
        
        IntPtr valPtr = config_get(cfg, "theme");
        Console.WriteLine($"Config Theme: {Marshal.PtrToStringAnsi(valPtr)}");
        free_string(valPtr);

        if (config_save(cfg, configPath)) {
            Console.WriteLine("Settings saved to disk.");
        }

        config_destroy(cfg);
        Console.WriteLine("\nDemo finished.");
    }
}
