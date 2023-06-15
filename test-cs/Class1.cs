using System;
using System.Runtime.InteropServices;

namespace test_cs
{
	[StructLayout(LayoutKind.Sequential)]
	public struct BasicStruct
	{
		public int wahoo;
		public byte small;
		public float foo;
	}

	public class Main
	{
		[UnmanagedCallersOnly(EntryPoint = "hello_world")]
		public static void HelloWorld() {
			Console.WriteLine("c# called from outside!");
		}

		[UnmanagedCallersOnly(EntryPoint = "process_value")]
		public static int ProcessValue(int x) => x*x;

		[UnmanagedCallersOnly(EntryPoint = "returns_struct")]
		public static BasicStruct ReturnsStruct() => new BasicStruct{
			wahoo = 5,
			small = 123,
			foo = 5.0f,
		};


		[UnmanagedCallersOnly(EntryPoint = "returns_string")]
		public static IntPtr ReturnsString() => Marshal.StringToCoTaskMemUTF8("I am a string from C#");

		[UnmanagedCallersOnly(EntryPoint = "cs_free")]
		public static void Free(IntPtr ptr) => Marshal.FreeCoTaskMem(ptr);

	}
}



 