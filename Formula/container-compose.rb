class ContainerCompose < Formula
  desc "A docker-compose like tool for Apple Containers "
  homepage "https://github.com/noghartt/container-compose"

  version "0.0.1-alpha.4"

  depends_on "socat"

  on_macos do
    on_arm do
      url "https://github.com/noghartt/container-compose/releases/download/v#{version}/container-compose-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "4be414aaa3aa9d13a52e45b6f0d99be1cfd12e91b205cf7435e212947e7d7039"
    end

    on_intel do
      url "https://github.com/noghartt/container-compose/releases/download/v#{version}/container-compose-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "320c5ca84e409629d053d39df84ec35916f3bafda10398fc215cd86d0fc97896"
    end
  end

  def install
    bin.install "container-compose"
  end
end