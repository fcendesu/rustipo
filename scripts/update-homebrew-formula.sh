#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 rustipo-v<version>" >&2
  exit 1
fi

tag="$1"
version="${tag#rustipo-v}"

if [[ "$version" == "$tag" || -z "$version" ]]; then
  echo "expected tag in the form rustipo-v<version>" >&2
  exit 1
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
formula_path="$repo_root/Formula/rustipo.rb"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

checksum_file="$tmpdir/${tag}-sha256sums.txt"
repo_slug="${RUSTIPO_GITHUB_REPO:-fcendesu/rustipo}"
checksum_url="https://github.com/${repo_slug}/releases/download/${tag}/${tag}-sha256sums.txt"

curl --fail --silent --show-error --location \
  --retry 3 \
  --output "$checksum_file" \
  "$checksum_url"

arm_sha="$(awk "/${tag}-aarch64-apple-darwin\\.tar\\.gz$/ { print \$1 }" "$checksum_file")"
intel_sha="$(awk "/${tag}-x86_64-apple-darwin\\.tar\\.gz$/ { print \$1 }" "$checksum_file")"

if [[ -z "$arm_sha" || -z "$intel_sha" ]]; then
  echo "missing macOS checksums in ${checksum_file}" >&2
  exit 1
fi

cat > "$formula_path" <<EOF
class Rustipo < Formula
  desc "Markdown-first static site generator for blogs, docs, notes, and personal sites"
  homepage "https://github.com/fcendesu/rustipo"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/fcendesu/rustipo/releases/download/${tag}/${tag}-aarch64-apple-darwin.tar.gz"
      sha256 "${arm_sha}"
    end

    on_intel do
      url "https://github.com/fcendesu/rustipo/releases/download/${tag}/${tag}-x86_64-apple-darwin.tar.gz"
      sha256 "${intel_sha}"
    end
  end

  def install
    binary = Dir["**/rustipo"].find { |path| File.file?(path) }
    odie "expected rustipo binary in release archive" if binary.nil?

    readme = Dir["**/README.md"].find { |path| File.file?(path) }
    license_file = Dir["**/LICENSE.md"].find { |path| File.file?(path) }

    bin.install binary => "rustipo"
    prefix.install readme if readme
    prefix.install license_file if license_file
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/rustipo --version")
  end
end
EOF

echo "updated ${formula_path} for ${tag}"
