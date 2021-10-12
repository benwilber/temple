class Temple < Formula
  version "0.4.0"
  desc "A commandline program that renders template files with structured context inputs. It is most often used to transform JSON data from a web API to a presentation format such as HTML."
  homepage "https://github.com/benwilber/temple"

  if OS.mac?
    url "https://github.com/benwilber/temple/releases/download/#{version}/temple-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 "7662bab79deaccdaeae1a88d74eb1e4345b56f0e08cd9669ff1cb1a79e2240f1"
  elsif OS.linux?
    url "https://github.com/benwilber/temple/releases/download/#{version}/temple-#{version}-x86_64-unknown-linux-musl.tar.gz"
    sha256 "98d9c88df4d4371c0626ac4d9d0c98ad8c290be6fb7d894f0884f886257a84bb"
  end

  def install
    bin.install "bin/temple"
  end
end
