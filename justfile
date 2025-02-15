alias b := build
alias r := release

default:
	@just --list

build:
	cargo build

release:
	cargo build --release
