use clap::Parser;
use wreq_util::Emulation;

#[derive(Parser, Debug)]
#[command(name = "netimitor", version, about)]
pub struct NetimitorConfig {
  #[arg(long, default_value = "127.0.0.1:8080")]
  pub address: String,

  #[arg(long, default_value = "chrome_137")]
  pub emulation: String,
}

impl Default for NetimitorConfig {
  fn default() -> Self {
    NetimitorConfig {
      address: "127.0.0.1:8080".to_string(),
      emulation: "chrome_137".to_string(),
    }
  }
}

impl NetimitorConfig {
  pub fn resolve_emulation(&self) -> Emulation {
    parse_emulation(&self.emulation)
  }
}

fn parse_emulation(s: &str) -> Emulation {
  match s {
    "chrome_100" => Emulation::Chrome100,
    "chrome_101" => Emulation::Chrome101,
    "chrome_104" => Emulation::Chrome104,
    "chrome_105" => Emulation::Chrome105,
    "chrome_106" => Emulation::Chrome106,
    "chrome_107" => Emulation::Chrome107,
    "chrome_108" => Emulation::Chrome108,
    "chrome_109" => Emulation::Chrome109,
    "chrome_110" => Emulation::Chrome110,
    "chrome_114" => Emulation::Chrome114,
    "chrome_116" => Emulation::Chrome116,
    "chrome_117" => Emulation::Chrome117,
    "chrome_118" => Emulation::Chrome118,
    "chrome_119" => Emulation::Chrome119,
    "chrome_120" => Emulation::Chrome120,
    "chrome_123" => Emulation::Chrome123,
    "chrome_124" => Emulation::Chrome124,
    "chrome_126" => Emulation::Chrome126,
    "chrome_127" => Emulation::Chrome127,
    "chrome_128" => Emulation::Chrome128,
    "chrome_129" => Emulation::Chrome129,
    "chrome_130" => Emulation::Chrome130,
    "chrome_131" => Emulation::Chrome131,
    "chrome_132" => Emulation::Chrome132,
    "chrome_133" => Emulation::Chrome133,
    "chrome_134" => Emulation::Chrome134,
    "chrome_135" => Emulation::Chrome135,
    "chrome_136" => Emulation::Chrome136,
    "chrome_137" => Emulation::Chrome137,
    _ => {
      eprintln!("unknown emulation \"{s}\", falling back to chrome_137");
      Emulation::Chrome137
    }
  }
}
