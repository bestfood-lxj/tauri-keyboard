#!/usr/bin/env just --justfile
set dotenv-load
set dotenv-filename := ".env.local"
default:
  just --list
get_jks:
	keytool -genkey -v -keystore my-release-key.jks -keyalg RSA -keysize 2048 -validity 10000 -alias my-alias
sign_reslear_apk:
	apksigner sign --ks my-release-key.jks --ks-key-alias my-alias --out my-app-signed.apk my-app-unsigned.apk
build_apk:
	cargo tauri android build  -t aarch64