# ClusterFuzzLite build image: the cargo-fuzz targets of fuzz/ are built inside it (see build.sh).
# Pinned by digest; Dependabot (docker ecosystem, see .github/dependabot.yml) keeps it current.
FROM gcr.io/oss-fuzz-base/base-builder-rust@sha256:3428d0a90b1e7efbe2ba947bd1d0c9a70594986b4b6553804547f1fb24cfc92f
COPY . $SRC/rust
COPY .clusterfuzzlite/build.sh $SRC/
WORKDIR $SRC/rust
