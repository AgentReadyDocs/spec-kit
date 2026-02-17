# Template formula for AgentReadyDocs `ard`.
#
# Intended home:
#   AgentReadyDocs/homebrew-tap (separate repo)
#
# Usage (once published in tap):
#   brew install agentreadydocs/tap/ard
#
# Update `url` and `sha256` per release.

class Ard < Formula
  desc "AgentReadyDocs CLI: lint agent-ready docs, install skills, and access templates/rubrics"
  homepage "https://github.com/AgentReadyDocs/spec-kit"
  version "0.1.0"

  on_macos do
    on_arm do
      url "https://github.com/AgentReadyDocs/spec-kit/releases/download/v0.1.0/ard-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_ME"
    end
    on_intel do
      url "https://github.com/AgentReadyDocs/spec-kit/releases/download/v0.1.0/ard-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_ME"
    end
  end

  on_linux do
    url "https://github.com/AgentReadyDocs/spec-kit/releases/download/v0.1.0/ard-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "REPLACE_ME"
  end

  def install
    bin.install "ard"
  end

  test do
    system "#{bin}/ard", "--help"
  end
end

