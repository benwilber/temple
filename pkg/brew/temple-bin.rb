class TempleBin < Formula
  version "0.2.1"
  desc "A commandline program that renders template files with structured context inputs. It is most often used to transform JSON data from a web API to a presentation format such as HTML."
  homepage "https://github.com/benwilber/temple"

  if OS.mac?
    url "https://github.com/benwilber/temple/releases/download/#{version}/temple-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 "7f2c405d210232d3097b7ceb82af1acb3be3e6e5c3cb0123007b80419222505d"
  elsif OS.linux?
    url "https://github.com/benwilber/temple/releases/download/#{version}/temple-#{version}-x86_64-unknown-linux-musl.tar.gz"
    sha256 "2067d9abe10ca57269c7667598561c2fc5e4aa640a47489c0e80a97da8f8ccbf"
  end

  def install
    bin.install "bin/temple"
  end
end
