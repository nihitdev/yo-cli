class Yoo < Formula
  desc "Local CLI for project, Git, and development environment information"
  homepage "https://github.com/nihitdev/yo-cli"
  version "1.0.0"
  license "GPL-3.0-or-later"

  on_macos do
    on_arm do
      url "https://github.com/nihitdev/yo-cli/releases/download/v1.0.0/yoo-v1.0.0-macos-aarch64.tar.gz"
      sha256 "76bc5805375d3d2413eec08744fb4b1733d1131068e7525379d3d3a58d000370"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/nihitdev/yo-cli/releases/download/v1.0.0/yoo-v1.0.0-linux-x86_64.tar.gz"
      sha256 "d09d5f0a5f471f30d6516e68f53ed134520909654958d073c8c6cae96197715d"
    end
  end

  def install
    bin.install "yoo"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/yoo --version")
  end
end
