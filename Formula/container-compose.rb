class ContainerCompose < Formula
  desc "A docker-compose like tool for Apple Containers "
  homepage "https://github.com/noghartt/container-compose"

  version "0.0.1-alpha.5"

  depends_on "socat"

  on_macos do
    on_arm do
      url "https://github.com/noghartt/container-compose/releases/download/v#{version}/container-compose-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "8b6f9d35b8a03eb14c059b347aa2b9e705e6c75c10496ba462a8cf7d58545973"
    end

    on_intel do
      url "https://github.com/noghartt/container-compose/releases/download/v#{version}/container-compose-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "a409ca44b8cec7ac540c0db453f6f8e3d22f29c6cbb4f8cb21ecfe946011610f"
    end
  end

  def install
    bin.install "container-compose"
  end
end