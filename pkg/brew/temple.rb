class Temple < Formula
  version "0.3.0"
  desc "A commandline program that renders template files with structured context inputs. It is most often used to transform JSON data from a web API to a presentation format such as HTML."
  homepage "https://github.com/benwilber/temple"

  if OS.mac?
    url "https://github.com/benwilber/temple/releases/download/#{version}/temple-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 "425e97bd31105f36d69095438055e2e2ca42dd7af32fb5d6dc10d4fe17dc3e72"
  elsif OS.linux?
    url "https://github.com/benwilber/temple/releases/download/#{version}/temple-#{version}-x86_64-unknown-linux-musl.tar.gz"
    sha256 "c43e407337105bf7716d2162efbc377dab8f7737475f5d08c0ed8ede0a08d4fb"
  end

  def install
    bin.install "bin/temple"
  end
end
