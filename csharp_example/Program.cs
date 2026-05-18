using System;
using System.Runtime.InteropServices;

class Program
{
    // Import the Rust library. 
    // The name depends on the OS (e.g., "libshared_std_rust_csharp.so" on Linux, "shared_std_rust_csharp.dll" on Windows)
    const string LibName = "libshared_std_rust_csharp";

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern int add_numbers(int a, int b);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr hello_rust(string name);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_string(IntPtr ptr);

    static void Main(string[] args)
    {
        // Test add_numbers
        int sum = add_numbers(10, 20);
        Console.WriteLine($"10 + 20 = {sum}");

        // Test hello_rust
        string name = "C# Developer";
        IntPtr ptr = hello_rust(name);
        
        if (ptr != IntPtr.Zero)
        {
            string result = Marshal.PtrToStringAnsi(ptr);
            Console.WriteLine(result);
            
            // Important: Free the memory allocated by Rust
            free_string(ptr);
        }
        else
        {
            Console.WriteLine("Failed to get greeting from Rust.");
        }
    }
}
