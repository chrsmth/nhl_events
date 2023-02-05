{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    #bildInputs = with pkgs; [ openssl ];
    nativeBuildInputs = with pkgs; [ pkg-config openssl openapi-generator-cli ];
}