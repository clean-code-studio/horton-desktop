cask "chatgpt" do
  version "0.12.0"
  arch = Hardware::CPU.arch.to_s
  sha256s = {
    "x86_64" => "d7f32d11f86ad8ac073dd451452124324e1c9154c318f15b77b5cd254000a3c4",
    "aarch64" => "c4c10eeb4a2c9a885da13047340372f461d411711c20472fc673fbf958bf6378"
  }
  if arch == "arm64" then arch = "aarch64" end
  url "https://github.com/clean-code-studio/horton-desktop/releases/download/v#{version}/ChatGPT_#{version}_macos_#{arch}.dmg"
  sha256 sha256s[arch]

  name "HortonDesktop"
  desc "Desktop wrapper for Horton Housing"
  homepage "https://github.com/clean-code-studio/horton-desktop#readme"

  app "HortonDesktop.app"

  uninstall quit: "com.horton.housing"

  zap trash: [
    "~/.horton",
    "~/Library/Caches/com.horton.housing",
    "~/Library/HTTPStorages/com.horton.housing.binarycookies",
    "~/Library/Preferences/com.horton.housing.plist",
    "~/Library/Saved Application State/com.horton.housing.savedState",
    "~/Library/WebKit/com.horton.housing",
  ]
end
