
const LIB_ROOT: &str = "test-cs/bin/Release/net7.0/win-x64/publish/";
const DEP_LIB_ROOT: &str = "C:/Users/patrick/.nuget/packages/runtime.win-x64.microsoft.dotnet.ilcompiler/7.0.5/sdk/";

fn main() -> anyhow::Result<()> {
	build_cs_project()?;

	println!("cargo:rustc-link-search=native={LIB_ROOT}");
	println!("cargo:rustc-link-search=native={DEP_LIB_ROOT}");

	println!("cargo:rustc-link-lib=test-cs");

	println!("cargo:rustc-link-lib=ole32");
	println!("cargo:rustc-link-lib=user32");

	println!("cargo:rustc-link-lib=static=bootstrapperdll");
	println!("cargo:rustc-link-lib=static=Runtime.WorkstationGC");
	println!("cargo:rustc-link-lib=static=System.Globalization.Native.Aot");
	println!("cargo:rustc-link-lib=static=System.IO.Compression.Native.Aot");
	println!("cargo:rustc-link-arg=/INCLUDE:NativeAOT_StaticInitialization");

	println!("cargo:warning=Running build.rs");

	Ok(())
}


use std::process::Command;

fn build_cs_project() -> anyhow::Result<()> {
	let status = Command::new("dotnet")
		.args(&["publish", "-r", "win-x64", "-c", "Release"])
		.current_dir("../test-cs")
		.status()?;

	if !status.success() {
		anyhow::bail!("C# build failed");
	}

	for entry in glob::glob("../test-cs/**/*.cs")? {
		if let Ok(path) = entry {
			println!("cargo:rerun-if-changed={}", path.display());
		}
	}

	for entry in glob::glob("../test-cs/*.csproj")? {
		if let Ok(path) = entry {
			println!("cargo:rerun-if-changed={}", path.display());
		}
	}

	Ok(())
}