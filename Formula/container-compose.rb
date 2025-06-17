class ContainerCompose < Formula
  desc "A docker-compose like tool for Apple Containers "
  homepage "https://github.com/noghartt/container-compose"

  version "0.0.1-alpha.3"

  depends_on "socat"

  on_macos do
    on_arm do
      url "https://github.com/noghartt/container-compose/releases/download/v#{version}/container-compose-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "330a99ce22ef2e6d9db8e37d4735479a4135441bb9d053cfd71de2e6cdce8d3a"
    end

    on_intel do
      url "https://github.com/noghartt/container-compose/releases/download/v#{version}/container-compose-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "e861f215d0ad23819fd27e485fe2c46f02e4446d7fc3b923c5cde05e6b8649d0"
    end
  end

  def install
    bin.install "container-compose"
  end
end