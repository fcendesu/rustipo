class Rustipo < Formula
  desc "Markdown-first static site generator for blogs, docs, notes, and personal sites"
  homepage "https://github.com/fcendesu/rustipo"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/fcendesu/rustipo/releases/download/rustipo-v0.11.0/rustipo-v0.11.0-aarch64-apple-darwin.tar.gz"
      sha256 "59fd0c20e15f9edab4d095b3d5015dd3bc530cd884b373d9244919ed03f5e2a2"
    end

    on_intel do
      url "https://github.com/fcendesu/rustipo/releases/download/rustipo-v0.11.0/rustipo-v0.11.0-x86_64-apple-darwin.tar.gz"
      sha256 "ae54ae2995d86ddda515df60d18cdaec82e839f90a1932bb7ae1477ab07655be"
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
