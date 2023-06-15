
const ILCOMPILER_PACKAGE_SDK: &str = "runtime.win-x64.microsoft.dotnet.ilcompiler/7.0.5/sdk/";

fn main() -> anyhow::Result<()> {
	build_cs_project()?;
	add_nuget_package_root_search_path()?;

	println!("cargo:rustc-link-lib=test-cs");

	println!("cargo:rustc-link-lib=ole32");
	println!("cargo:rustc-link-lib=user32");

	println!("cargo:rustc-link-lib=static=bootstrapperdll");
	println!("cargo:rustc-link-lib=static=Runtime.WorkstationGC");
	println!("cargo:rustc-link-lib=static=System.Globalization.Native.Aot");
	println!("cargo:rustc-link-lib=static=System.IO.Compression.Native.Aot");
	println!("cargo:rustc-link-arg=/INCLUDE:NativeAOT_StaticInitialization");

	Ok(())
}


use std::process::Command;

fn add_nuget_package_root_search_path() -> anyhow::Result<()> {
	let output = Command::new("dotnet")
		.args(&["nuget", "locals", "global-packages", "--list"])
		.output()?;

	let output_str = String::from_utf8_lossy(&output.stdout);

	let (_, path) = output_str.trim().split_once(": ")
		.expect("Unexpected output from dotnet nuget locals");

	println!("cargo:rustc-link-search=native={path}/{ILCOMPILER_PACKAGE_SDK}");

	Ok(())
}

fn build_cs_project() -> anyhow::Result<()> {
	let out_dir = std::env::var_os("OUT_DIR").unwrap();
	let out_dir = out_dir.to_str().unwrap();

	let status = Command::new("dotnet")
		.args(&[
			"publish",
			"-r", "win-x64",
			"-c", "Release",
			&format!("-p:OutputPath={out_dir}/bin/"),
			&format!("-p:BaseIntermediateOutputPath={out_dir}/obj/"),
			&format!("-p:PublishDir={out_dir}/publish/"),
		])
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

	println!("cargo:rustc-link-search=native={out_dir}/publish");

	Ok(())
}