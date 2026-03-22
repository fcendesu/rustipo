class Rustipo < Formula
  desc "Markdown-first static site generator for blogs, docs, notes, and personal sites"
  homepage "https://github.com/fcendesu/rustipo"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/fcendesu/rustipo/releases/download/rustipo-v0.10.0/rustipo-v0.10.0-aarch64-apple-darwin.tar.gz"
      sha256 "3cd60a98edb00fe379bebaae1fa1b1e934f95964948dc768cdbae41443da8712"
    end

    on_intel do
      url "https://github.com/fcendesu/rustipo/releases/download/rustipo-v0.10.0/rustipo-v0.10.0-x86_64-apple-darwin.tar.gz"
      sha256 "e8549cba5af4decbebe5686908ca2ce5d944dfc7a274314e4cdc327d1296b5a3"
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
