using System;
using System.Runtime.InteropServices;
using Xunit;

public class LibraryTests
{
    const string LibName = "libshared_std_rust_csharp";

    // --- P/Invoke Signatures ---
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern int math_add(int a, int b);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern double parallel_sum(IntPtr data, int len);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void parallel_square(IntPtr data, int len);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern bool regex_is_match(string pattern, string text);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr regex_replace(string pattern, string text, string repl);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void register_event_callback(IntPtr callback);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void trigger_rust_event(string message);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr crypto_sha256(string input);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr crypto_encode_base64(string input);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_string(IntPtr ptr);

    // --- Tests ---

    [Fact]
    public void TestParallelSum()
    {
        double[] data = { 1.0, 2.0, 3.0, 4.0 };
        GCHandle handle = GCHandle.Alloc(data, GCHandleType.Pinned);
        try {
            double sum = parallel_sum(handle.AddrOfPinnedObject(), data.Length);
            Assert.Equal(10.0, sum);
        } finally {
            handle.Free();
        }
    }

    [Fact]
    public void TestParallelSquare()
    {
        double[] data = { 1.0, 2.0, 3.0, 4.0 };
        GCHandle handle = GCHandle.Alloc(data, GCHandleType.Pinned);
        try {
            parallel_square(handle.AddrOfPinnedObject(), data.Length);
            Assert.Equal(1.0, data[0]);
            Assert.Equal(4.0, data[1]);
            Assert.Equal(9.0, data[2]);
            Assert.Equal(16.0, data[3]);
        } finally {
            handle.Free();
        }
    }

    [Fact]
    public void TestRegex()
    {
        Assert.True(regex_is_match(@"^\d+$", "12345"));
        Assert.False(regex_is_match(@"^\d+$", "123a5"));
        
        IntPtr ptr = regex_replace("foo", "foo bar foo", "baz");
        try {
            Assert.Equal("baz bar baz", Marshal.PtrToStringAnsi(ptr));
        } finally {
            free_string(ptr);
        }
    }

    [Fact]
    public void TestCrypto()
    {
        IntPtr hashPtr = crypto_sha256("hello");
        Assert.Equal("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824", Marshal.PtrToStringAnsi(hashPtr));
        free_string(hashPtr);

        IntPtr b64Ptr = crypto_encode_base64("hello");
        Assert.Equal("aGVsbG8=", Marshal.PtrToStringAnsi(b64Ptr));
        free_string(b64Ptr);
    }

    [Fact]
    public void TestEventBridge()
    {
        bool wasCalled = false;
        // This delegate must be kept alive to prevent GC
        EventDelegate callback = (ptr) => {
            wasCalled = true;
            free_string(ptr); // Rust sends a string, we must free it
        };

        IntPtr callbackPtr = Marshal.GetFunctionPointerForDelegate(callback);
        register_event_callback(callbackPtr);
        
        trigger_rust_event("Event Triggered!");
        Assert.True(wasCalled);
    }

    delegate void EventDelegate(IntPtr message);
}
