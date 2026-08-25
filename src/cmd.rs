use clap::Parser;
use wreq_util::Profile;

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
  pub fn resolve_profile(&self) -> Profile {
    parse_profile(&self.emulation)
  }
}

fn parse_profile(s: &str) -> Profile {
  match s {
    "chrome_100" => Profile::Chrome100,
    "chrome_101" => Profile::Chrome101,
    "chrome_104" => Profile::Chrome104,
    "chrome_105" => Profile::Chrome105,
    "chrome_106" => Profile::Chrome106,
    "chrome_107" => Profile::Chrome107,
    "chrome_108" => Profile::Chrome108,
    "chrome_109" => Profile::Chrome109,
    "chrome_110" => Profile::Chrome110,
    "chrome_114" => Profile::Chrome114,
    "chrome_116" => Profile::Chrome116,
    "chrome_117" => Profile::Chrome117,
    "chrome_118" => Profile::Chrome118,
    "chrome_119" => Profile::Chrome119,
    "chrome_120" => Profile::Chrome120,
    "chrome_123" => Profile::Chrome123,
    "chrome_124" => Profile::Chrome124,
    "chrome_126" => Profile::Chrome126,
    "chrome_127" => Profile::Chrome127,
    "chrome_128" => Profile::Chrome128,
    "chrome_129" => Profile::Chrome129,
    "chrome_130" => Profile::Chrome130,
    "chrome_131" => Profile::Chrome131,
    "chrome_132" => Profile::Chrome132,
    "chrome_133" => Profile::Chrome133,
    "chrome_134" => Profile::Chrome134,
    "chrome_135" => Profile::Chrome135,
    "chrome_136" => Profile::Chrome136,
    "chrome_137" => Profile::Chrome137,
    "edge_101" => Profile::Edge101,
    "edge_122" => Profile::Edge122,
    "edge_127" => Profile::Edge127,
    "edge_131" => Profile::Edge131,
    "edge_134" => Profile::Edge134,
    "safari_ios_17_2" => Profile::SafariIos17_2,
    "safari_ios_17_4_1" => Profile::SafariIos17_4_1,
    "safari_ios_16_5" => Profile::SafariIos16_5,
    "safari_15_3" => Profile::Safari15_3,
    "safari_15_5" => Profile::Safari15_5,
    "safari_15_6_1" => Profile::Safari15_6_1,
    "safari_16" => Profile::Safari16,
    "safari_16_5" => Profile::Safari16_5,
    "safari_17_0" => Profile::Safari17_0,
    "safari_17_2_1" => Profile::Safari17_2_1,
    "safari_17_4_1" => Profile::Safari17_4_1,
    "safari_17_5" => Profile::Safari17_5,
    "safari_18" => Profile::Safari18,
    "safari_ipad_18" => Profile::SafariIPad18,
    "safari_18_2" => Profile::Safari18_2,
    "safari_ios_18_1_1" => Profile::SafariIos18_1_1,
    "safari_18_3" => Profile::Safari18_3,
    "safari_18_3_1" => Profile::Safari18_3_1,
    "safari_18_5" => Profile::Safari18_5,
    "okhttp_3_9" => Profile::OkHttp3_9,
    "okhttp_3_11" => Profile::OkHttp3_11,
    "okhttp_3_13" => Profile::OkHttp3_13,
    "okhttp_3_14" => Profile::OkHttp3_14,
    "okhttp_4_9" => Profile::OkHttp4_9,
    "okhttp_4_10" => Profile::OkHttp4_10,
    "okhttp_4_12" => Profile::OkHttp4_12,
    "okhttp_5" => Profile::OkHttp5,
    "firefox_109" => Profile::Firefox109,
    "firefox_117" => Profile::Firefox117,
    "firefox_128" => Profile::Firefox128,
    "firefox_133" => Profile::Firefox133,
    "firefox_135" => Profile::Firefox135,
    "firefox_private_135" => Profile::FirefoxPrivate135,
    "firefox_android_135" => Profile::FirefoxAndroid135,
    "firefox_136" => Profile::Firefox136,
    "firefox_private_136" => Profile::FirefoxPrivate136,
    "firefox_139" => Profile::Firefox139,
    "opera_116" => Profile::Opera116,
    "opera_117" => Profile::Opera117,
    "opera_118" => Profile::Opera118,
    "opera_119" => Profile::Opera119,
    _ => {
      eprintln!("unknown emulation \"{s}\", falling back to chrome_137");
      Profile::Chrome137
    }
  }
}
