# NetImitor (Network Impersonator)

A forward HTTP proxy with browser emulation, built with [actix-web](https://actix.rs/) and [wreq](https://crates.io/crates/wreq).

## Usage

```bash
# Start the proxy
cargo run

# With custom address and emulation profile
cargo run -- --address 0.0.0.0:8080 --emulation chrome_137

# View available options
cargo run -- --help
```

```
Usage: netimitor [OPTIONS]

Options:
      --address <ADDRESS>      [default: 127.0.0.1:8080]
      --emulation <EMULATION>  [default: chrome_137]
  -h, --help                   Print help
  -V, --version                Print version
```

### Making requests

Forward requests through the proxy by placing the target URL in the path:

```bash
curl http://localhost:8080/http://example.com
```

Note that:

- The proxy also supports `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`, and `OPTIONS` methods. Sensitive headers (`Authorization`, `Cookie`, `X-Api-Key`, `X-Auth-Token`) are forwarded
- Hop-by-hop headers (`Transfer-Encoding`, `Connection`, `Keep-Alive`) are stripped from responses.

### Endpoints

| Path       | Method | Description         |
| ---------- | ------ | ------------------- |
| `/healthz` | GET    | Health check        |
| `/*`       | *      | Proxy to target URL |

## Emulation profiles

Defaults to `chrome_137`.

| Browser | Versions |
| ------- | -------- |
| Chrome | `chrome_100`, `chrome_101`, `chrome_104`–`chrome_110`, `chrome_114`, `chrome_116`–`chrome_120`, `chrome_123`, `chrome_124`, `chrome_126`–`chrome_137` |
| Edge | `edge_101`, `edge_122`, `edge_127`, `edge_131`, `edge_134` |
| Safari | `safari_ios_17_2`, `safari_ios_17_4_1`, `safari_ios_16_5`, `safari_15_3`, `safari_15_5`, `safari_15_6_1`, `safari_16`, `safari_16_5`, `safari_17_0`, `safari_17_2_1`, `safari_17_4_1`, `safari_17_5`, `safari_18`, `safari_ipad_18`, `safari_18_2`, `safari_18_1_1`, `safari_18_3`, `safari_18_3_1` |
| Firefox | `firefox_109`, `firefox_117`, `firefox_128`, `firefox_133`, `firefox_135`, `firefox_private_135`, `firefox_android_135`, `firefox_136`, `firefox_private_136`, `firefox_139` |
| Opera | `opera_116`, `opera_117`, `opera_118`, `opera_119` |
| OkHttp | `okhttp_3_9`, `okhttp_3_11`, `okhttp_3_13`, `okhttp_3_14`, `okhttp_4_9`, `okhttp_4_10`, `okhttp_4_12`, `okhttp_5` |

## Docker

```bash
docker build -t netimitor .
docker run -p 8080:8080 netimitor --address 0.0.0.0:8080
```

## License

This project is licensed under the [MIT License](LICENSE).

## Credits

- This project is inspired by [curl-impersonate](https://github.com/lwthiker/curl-impersonate). Special thanks to the author for the concept of browser impersonation via HTTP client.

- The core HTTP client functionality is powered by [wreq](https://github.com/0x676e67/wreq) and [wreq-util](https://github.com/0x676e67/wreq-util), which provide TLS and HTTP/2 fingerprint emulation for browser impersonation.

## Disclaimer
This project is assisted by LLM, with the list of models used are defined below:

- `Assisted-by: OpenCode:deepseek-v4-flash`
