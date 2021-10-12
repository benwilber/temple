class Temple < Formula
  version "0.4.1"
  desc "A commandline program that renders template files with structured context inputs. It is most often used to transform JSON data from a web API to a presentation format such as HTML."
  homepage "https://github.com/benwilber/temple"

  if OS.mac?
    url "https://github.com/benwilber/temple/releases/download/#{version}/temple-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 "ca82e40f7f3ecf0fb47d4cbd26f17724f42193b6ccdc8c361818514b4f84ee92"
  elsif OS.linux?
    url "https://github.com/benwilber/temple/releases/download/#{version}/temple-#{version}-x86_64-unknown-linux-musl.tar.gz"
    sha256 "6b4e3c3ec3997c2f0eafbdf2a667cdf11d66a73e6b47cb03f939ac7ba3a3eb3f"
  end

  def install
    bin.install "bin/temple"
  end
end
