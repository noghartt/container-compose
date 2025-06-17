class ContainerCompose < Formula
  desc "A docker-compose like tool for Apple Containers "
  homepage "https://github.com/noghartt/container-compose"

  version "0.0.1-alpha.3"

  depends_on "socat"

  on_macos do
    on_arm do
      url "https://github.com/noghartt/container-compose/releases/download/v#{version}/container-compose-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "b4a5a33d2399762c5d0c30ea1bbb3390e62f6cce96065ee382c9cbca93e65a0a"
    end

    on_intel do
      url "https://github.com/noghartt/container-compose/releases/download/v#{version}/container-compose-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "03ccd4c0a18df718bab7e591017a3220dbb6aaf440cc631104c3cf9639ca1ce5"
    end
  end

  def install
    bin.install "container-compose"
  end
end