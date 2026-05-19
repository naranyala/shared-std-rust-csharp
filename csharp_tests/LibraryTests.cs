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
    public static extern int math_multiply(int a, int b);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr text_to_uppercase(string name);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr text_reverse(string name);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr session_create(uint id, string name);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void session_add_score(IntPtr session, int points);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern int session_get_score(IntPtr session);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void session_destroy(IntPtr session);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_string(IntPtr ptr);

    // --- Tests ---

    [Fact]
    public void TestMathAdd()
    {
        Assert.Equal(30, math_add(10, 20));
        Assert.Equal(0, math_add(-10, 10));
    }

    [Fact]
    public void TestMathMultiply()
    {
        Assert.Equal(200, math_multiply(10, 20));
        Assert.Equal(-20, math_multiply(10, -2));
    }

    [Fact]
    public void TestTextUppercase()
    {
        string input = "hello rust";
        IntPtr ptr = text_to_uppercase(input);
        try
        {
            string result = Marshal.PtrToStringAnsi(ptr);
            Assert.Equal("HELLO RUST", result);
        }
        finally
        {
            free_string(ptr);
        }
    }

    [Fact]
    public void TestTextReverse()
    {
        string input = "rust";
        IntPtr ptr = text_reverse(input);
        try
        {
            string result = Marshal.PtrToStringAnsi(ptr);
            Assert.Equal("tsur", result);
        }
        finally
        {
            free_string(ptr);
        }
    }

    [Fact]
    public void TestSessionLifecycle()
    {
        IntPtr session = session_create(123, "TestUser");
        try
        {
            Assert.Equal(0, session_get_score(session));
            
            session_add_score(session, 100);
            Assert.Equal(100, session_get_score(session));
            
            session_add_score(session, -50);
            Assert.Equal(50, session_get_score(session));
        }
        finally
        {
            session_destroy(session);
        }
    }
}
