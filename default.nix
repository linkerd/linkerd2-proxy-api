scope@{ pkgs ? import <nixpkgs> { } }:

pkgs.buildEnv {
  name = "linkerd2-proxy-env";
  paths = with pkgs; [
    git
    bash
    direnv
    binutils
    stdenv
    bashInteractive
    docker
    docker-compose
    cacert
    gcc
    cmake
    rustup
    pkg-config
    openssl
    protobuf
    rdkafka
    bundler
    docker
    (glibcLocales.override { locales = [ "en_US.UTF-8" ]; })
    yarn
    openssl
    perl
    remarshal
    snappy
    gnumake
    autoconf
  ];
  passthru = {
    PROTOC = "${pkgs.protobuf}/bin/protoc";
    PROTOC_INCLUDE = "${pkgs.protobuf}/include";
    LOCALE_ARCHIVE = "${pkgs.glibcLocales}/lib/locale/locale-archive";
    LC_ALL = "en_US.UTF-8";
    OPENSSL_DIR = "${pkgs.openssl.dev}";
    OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
    SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
    GIT_SSL_CAINFO = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
    CURL_CA_BUNDLE = "${pkgs.cacert}/etc/ca-bundle.crt";
    CARGO_TERM_COLOR = "always";
    AWS_ACCESS_KEY_ID = "dummy";
    AWS_SECRET_ACCESS_KEY = "dummy";
    RUST_BACKTRACE = "full";
  };
}
