class Temple < Formula
  version "0.3.1"
  desc "A commandline program that renders template files with structured context inputs. It is most often used to transform JSON data from a web API to a presentation format such as HTML."
  homepage "https://github.com/benwilber/temple"

  if OS.mac?
    url "https://github.com/benwilber/temple/releases/download/#{version}/temple-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 "3bd628d91b74cc53ab06119c818da1ddbcb342190feb2b75c783144b5235d5d4"
  elsif OS.linux?
    url "https://github.com/benwilber/temple/releases/download/#{version}/temple-#{version}-x86_64-unknown-linux-musl.tar.gz"
    sha256 "da519505de98116ac417f70a425bc9c4c8a909919fb6abfcce2b0e197d296abf"
  end

  def install
    bin.install "bin/temple"
  end
end
