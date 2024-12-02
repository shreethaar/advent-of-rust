{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
	buildInputs = with pkgs; [
	binutils
	coreutils
	gnumake
	gcc
	pkg-config
	cmake
	autoconf
	libtool
	cargo
	rustc
	rustfmt
	rust-analyzer
	clippy
	];

	shellHook = ''
		echo "Rust version: $(rustc --version)"
		export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/library"
		'';
}
