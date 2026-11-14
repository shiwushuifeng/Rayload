# Rayload Plugin Development Guide / Rayload 插件开发指南

Rayload supports two plugin shapes:

- **Rule plugins** (`type: "rule"`) are declarative JSON manifests executed by
  the Rust host. They are the recommended choice for anything that needs the
  network, cookies or custom headers.
- **Script plugins** (`type: "script"`) run a `.sh` file in a child process and
  speak a newline-delimited JSON protocol. They are powerful but execute local
  code, so Rayload shows a red high-risk warning before import.

Rayload 支持两类插件：

- **规则插件**（`type: "rule"`）：声明式 JSON 清单，由 Rust 宿主执行。需要网络、
  Cookie 或自定义请求头时，应优先使用规则插件。
- **脚本插件**（`type: "script"`）：在独立子进程中运行 `.sh` 文件，通过逐行
  JSON 协议通信。脚本可以执行本地代码，因此导入时会显示红色高危警告。

The normative manifest definition lives in [`plugin.schema.json`](../plugin.schema.json).
规范的清单定义见 [`plugin.schema.json`](../plugin.schema.json)。

## Manifest schema / 清单结构

Every rule plugin is a single JSON object validated against
[`plugin.schema.json`](../plugin.schema.json). The minimal shape is:

每个规则插件都是一个 JSON 对象，并由 [`plugin.schema.json`](../plugin.schema.json)
校验。最小结构如下：

```json
{
  "name": "Example parser",
  "version": "1.0.0",
  "description": "Resolve a share link",
  "type": "rule",
  "runtime": "http",
  "permissions": ["network", "cookies", "modify_headers"],
  "allowedDomains": ["pan.example.com", "*.example-cdn.com"],
  "grantedPermissions": ["network", "cookies", "modify_headers"],
  "testUrl": "https://pan.example.com/s/abc123",
  "rules": [
    {
      "match": "^https://pan\\.example\\.com/s/([A-Za-z0-9]+)",
      "request": {
        "method": "POST",
        "url": "https://pan.example.com/api/share/{1}/verify",
        "headers": { "Referer": "https://pan.example.com/" },
        "body": "password={{password}}&share={1}"
      },
      "extract": { "json": "data.downloadUrl" }
    }
  ]
}
```

| Part | Purpose / 作用 |
| --- | --- |
| `type` | `rule` (JSON only) or `script` (`.sh` entry). / 规则或脚本。 |
| `runtime` | `http`, `download`, `script` or `echo`. / 执行方式。 |
| `permissions` | Capabilities the plugin requests. / 插件申请的能力。 |
| `grantedPermissions` | Subset written by Rayload after user consent. / 用户同意后由 Rayload 写入。 |
| `allowedDomains` | Exact hosts or `*.subdomain` entries; required with `network`. / 域名白名单。 |
| `rules[]` | Ordered `match` → `request` → `extract` rules. / 有序匹配、请求与提取规则。 |

For `runtime: "download"`, use `url` or `repository` + `asset` instead of
`rules`. For script plugins, add `entry` pointing to a sibling `.sh` file.

`runtime: "download"` 使用 `url` 或 `repository` + `asset`；脚本插件使用
`entry` 指向同目录的 `.sh` 文件。

## 1. Trust model / 信任模型

Rayload follows Android-style least privilege:

1. A plugin **declares** the capabilities it wants in `permissions`.
2. The import dialog explains each capability and colour-codes it by risk.
3. The user grants a subset. Rayload writes that subset to
   `grantedPermissions`; a manifest cannot grant itself anything.
4. At runtime the Rust host intersects `permissions` and `grantedPermissions`
   again, checks every URL against `allowedDomains`, and strips unsafe headers.

Rayload 采用类似 Android 的最小权限模型：

1. 插件在 `permissions` 中**声明**所需能力。
2. 导入对话框逐项解释权限，并按风险级别着色。
3. 用户授予其中一个子集，Rayload 将其写入 `grantedPermissions`；清单无法自行提权。
4. 运行时 Rust 宿主再次取声明与授予权限的交集，逐请求检查 `allowedDomains`，
   并强制剥离不安全请求头。

## 2. Permission scopes / 权限作用域

| Permission | Purpose / 用途 | Risk / 风险 | Runtime rule / 运行时规则 |
| --- | --- | --- | --- |
| `network` | Send HTTP(S) requests to declared hosts / 向声明域名发送 HTTP(S) 请求 | Normal / 常规 | Must be paired with a non-empty `allowedDomains`; every request is checked. / 必须配合非空 `allowedDomains`，每个请求都会校验。 |
| `cookies` | Read the app's saved cloud-drive cookies / 读取 App 保存的网盘 Cookie | **High / 高危** | Requires `network`; only cookies for allowed hosts are injected. / 必须同时声明 `network`，且只注入允许域名的 Cookie。 |
| `modify_headers` | Set custom `User-Agent` / `Referer` / 自定义 `User-Agent`、`Referer` | Medium / 中危 | Without it both headers are removed, even if the rule supplies them. / 未授权时即使规则写了也会被移除。 |
| `read_clipboard` | Read clipboard text / 读取剪贴板文本 | High / 高危 | Denied by default; only the host clipboard API checks this grant. / 默认拒绝，仅宿主剪贴板 API 会检查该授权。 |

### 2.1 Declaring permissions / 声明权限

```json
{
  "name": "Example parser",
  "type": "rule",
  "runtime": "http",
  "permissions": ["network", "cookies", "modify_headers"],
  "allowedDomains": ["pan.example.com", "*.example-cdn.com"],
  "grantedPermissions": ["network", "cookies", "modify_headers"]
}
```

`allowedDomains` accepts exact hosts (`pan.example.com`) and wildcard
subdomains (`*.example-cdn.com`). Schemes, ports, paths, userinfo, IP literals
and bare `*` are rejected. A wildcard matches subdomains but **not** the root
domain.

`allowedDomains` 支持精确域名（`pan.example.com`）和通配子域名
（`*.example-cdn.com`）。协议、端口、路径、用户信息、IP 字面量和裸 `*` 都会被
拒绝。通配符只匹配子域名，**不匹配**根域名。

### 2.2 How enforcement works / 强制拦截规则

- No `network` grant: every host request fails before DNS or TCP.
- A URL outside `allowedDomains`: rejected with `plugin may not access <host>`.
- A rule supplies `Cookie`: the header is **always discarded**. When `cookies`
  is granted, the host injects only the cookie scoped to the request host and
  its declared parent domain.
- A rule supplies `User-Agent` or `Referer` without `modify_headers`: both are
  removed. `Host`, `Content-Length`, `Connection`, `Transfer-Encoding` and
  `Proxy-Authorization` are always stripped.
- Redirects are not followed automatically, so a request cannot escape the
  domain allow-list through a redirect.

- 未授予 `network`：任何请求都会在 DNS/TCP 之前失败。
- URL 不在 `allowedDomains`：直接拒绝并返回 `plugin may not access <host>`。
- 规则自行提供 `Cookie`：该请求头**始终丢弃**。只有授予 `cookies` 后，宿主才会
  注入与请求域名（及其声明的父域）匹配的 Cookie。
- 未授予 `modify_headers` 时，规则提供的 `User-Agent`、`Referer` 会被移除；
  `Host`、`Content-Length`、`Connection`、`Transfer-Encoding`、
  `Proxy-Authorization` 永远由宿主管理。
- 默认不自动跟随重定向，避免通过 302 跳出域名白名单。

### 2.3 Sandbox boundary / 沙箱边界

- Rule plugins are pure JSON. They cannot read files, spawn processes or run
  shell commands; the host performs the HTTP request on their behalf.
- There is no direct socket access. A request without `network`, outside
  `allowedDomains`, or requiring a redirect fails before reaching the target.
- Script plugins are **not** OS-sandboxed: they run as a child process with the
  app's file permissions. Treat them as trusted local code and review the source
  before importing.
- The host never grants a capability because the manifest lists it. Only the
  user-approved `grantedPermissions` subset is effective.

- 规则插件是纯 JSON：不能读文件、启动进程或执行 shell，HTTP 请求由宿主代为发出。
- 插件没有直接 socket 权限；未授予 `network`、域名不在白名单或需要重定向的
  请求都会在触达目标前失败。
- 脚本插件**不在操作系统沙箱中**：它以子进程运行，拥有应用的文件权限。请将其
  视为受信任的本地代码，导入前审查源码。
- 宿主不会因为清单声明就授予权限；只有用户批准的 `grantedPermissions` 子集生效。

### 2.4 Cookie store / Cookie 存储

The host reads `plugin-cookies.json` from the app local data directory. The file
is a flat host-to-cookie map:

宿主从应用本地数据目录读取 `plugin-cookies.json`，格式为“域名 → Cookie 字符串”：

```json
{
  "pan.example.com": "sid=...; uid=...",
  "example.com": "theme=dark"
}
```

Cookie lookup prefers the exact host and then walks parent domains. A plugin
never receives cookies for hosts outside its `allowedDomains`.

查找时优先精确域名，再逐级查找父域。插件永远拿不到 `allowedDomains` 之外的
Cookie。

## 3. Rule plugin manifest / 规则插件清单

### 3.1 Top-level fields / 顶层字段

| Field | Required | Notes / 说明 |
| --- | --- | --- |
| `name` | Yes | Shown in the plugin list. / 插件列表显示名称。 |
| `type` | Yes | `rule` or `script`. / 规则或脚本。 |
| `runtime` | Yes | `http`, `download`, `script` or `echo`. / 运行方式。 |
| `version` | No | SemVer recommended. / 建议使用 SemVer。 |
| `description` | No | Short list subtitle. / 列表副标题。 |
| `permissions` | No | Declared capabilities. / 声明的能力。 |
| `allowedDomains` | With `network` | Host allow-list. / 域名白名单。 |
| `grantedPermissions` | No | Written by Rayload after consent. / 用户同意后由 Rayload 写入。 |
| `testUrl` | No | URL used by the Test button for `runtime: http`. / HTTP 规则测试地址。 |
| `rules` | For `runtime: http` | Ordered rule list. / 有序规则列表。 |
| `url` | For `runtime: download` | Direct `https://github.com/<owner>/<repo>/releases/download/` URL. / 直链下载地址。 |
| `repository` | Alternative | `owner/repo`; the newest GitHub release is resolved when Test runs. / 测试时解析最新 Release。 |
| `asset` | No | Substring used to pick the APK from that release, for example `arm64`. / 用于挑选 APK 的文件名片段。 |

### 3.2 A complete POST + extraction-code example / 完整的 POST 提取码示例

This rule matches a share URL, POSTs the extraction code, and reads the download
URL from a JSON response. `{1}` is the first regex capture; `{{pwd}}` is a
caller-supplied variable.

下面的规则匹配分享链接，POST 提取码，再从 JSON 响应中读取下载地址。
`{1}` 是第一个正则捕获组，`{{pwd}}` 是调用方传入的变量。

```json
{
  "name": "Example cloud share parser",
  "version": "1.0.0",
  "description": "POSTs an extraction code and extracts the CDN URL.",
  "type": "rule",
  "runtime": "http",
  "permissions": ["network", "cookies", "modify_headers"],
  "allowedDomains": ["pan.example.com", "*.example-cdn.com"],
  "grantedPermissions": ["network", "cookies", "modify_headers"],
  "testUrl": "https://pan.example.com/s/abc123",
  "rules": [
    {
      "match": "^https://pan\\.example\\.com/s/([A-Za-z0-9]+)",
      "request": {
        "method": "POST",
        "url": "https://pan.example.com/api/share/{1}/verify",
        "headers": {
          "Content-Type": "application/x-www-form-urlencoded",
          "Referer": "https://pan.example.com/"
        },
        "body": "pwd={{pwd}}&share={1}"
      },
      "extract": {
        "json": "data.downloadUrl"
      }
    }
  ]
}
```

Host call / 宿主调用：

```ts
const result = await invoke('plugin_resolve', {
  file: 'example-parser.json',
  url: 'https://pan.example.com/s/abc123',
  variables: { pwd: '1234' },
})
// { status: 200, downloadUrl: 'https://cdn.example-cdn.com/file.zip' }
```

Extraction may use a JSON dot path (`data.list.0.url`), or a regex with a
capture group:

提取也可以使用正则捕获组：

```json
{
  "extract": {
    "regex": "downloadUrl\\s*=\\s*\"(https://[^\"]+)\"",
    "group": 1
  }
}
```

Rules are evaluated in order; the first `match` wins. A request without
`network` permission, outside `allowedDomains`, or with a redirect cannot reach
the network.

规则按顺序匹配，第一个命中的规则生效。未授予 `network`、域名不在白名单或需要
跟随重定向的请求都无法访问网络。

### 3.3 Bundled sample: download Rayload itself / 内置示例：下载 Rayload 本体

The **Sample** button installs this manifest. It demonstrates a download-only
plugin that never parses a page: the host asks the GitHub release API for the
newest Rayload package matching the current ABI and hands the URL to Aria2.

点击 **示例** 按钮会安装下面的清单。它演示只做下载的插件：宿主向 GitHub Release API
查询与当前 ABI 匹配的最新 Rayload 安装包，然后把地址交给 Aria2。

```json
{
  "name": "Rayload self-download test",
  "type": "rule",
  "runtime": "download",
  "permissions": ["network"],
  "allowedDomains": ["api.github.com", "github.com"],
  "grantedPermissions": ["network"],
  "repository": "shiwushuifeng/Rayload",
  "asset": "arm64"
}
```

### 3.4 Template variables / 模板变量

`url`, header values and `body` support two placeholder forms:

`url`、请求头值与 `body` 支持两种占位符：

- `{1}`, `{2}`, … — regex capture groups from the rule's `match`.
- `{{name}}` — a caller-supplied variable. `{{password}}` is the conventional
  name for a share password, for example `body: "pwd={{password}}"`.

Variables are substituted only inside the request template; they are not
written to logs. Use them for share passwords or extraction codes, never embed
credentials in the manifest. The caller decides which variables to pass; the
host does not invent or persist them.

`{1}`、`{2}` 等是 `match` 的正则捕获组；`{{name}}` 是调用方传入的变量，
`{{password}}` 是分享密码的约定名称，例如 `body: "pwd={{password}}"`。
变量只在请求模板内替换，不会写入日志；不要把账号密码硬编码进清单。调用方决定
传入哪些变量，宿主不会自行生成或持久化。

```json
{
  "request": {
    "method": "POST",
    "url": "https://pan.example.com/api/share/{1}/verify",
    "headers": { "Content-Type": "application/x-www-form-urlencoded" },
    "body": "pwd={{password}}&share={1}"
  }
}
```

## 4. Script plugins / 脚本插件

### 4.1 Files / 文件结构

```text
plugins/
├── my-plugin.json
└── my-plugin.sh
```

```json
{
  "name": "My script plugin",
  "type": "script",
  "runtime": "script",
  "entry": "my-plugin.sh",
  "permissions": [],
  "allowedDomains": [],
  "grantedPermissions": []
}
```

Standalone `.sh` imports create a manifest automatically and grant no
privileged capabilities. Use a JSON manifest when the script needs a declared
capability set.

直接导入 `.sh` 会自动生成清单，且不授予任何特权能力。需要声明能力时，请使用
JSON 清单。

### 4.2 JSON line protocol / 逐行 JSON 协议

The host writes one JSON object per line to stdin and reads one JSON object per
line from stdout. The first exchange is always a version handshake:

宿主向 stdin 写入一行一个 JSON，并从 stdout 读取一行一个 JSON。第一次通信固定为
版本握手：

```text
host  -> {"method":"handshake","version":1}
plugin<- {"version":1}
```

Then the host sends a request and expects the same object echoed or a result:

```text
host  -> {"id":1,"method":"echo","plugin":"My script plugin","version":1}
plugin<- {"id":1,"method":"echo","plugin":"My script plugin","version":1}
```

A minimal script / 最小脚本：

```sh
#!/system/bin/sh
while IFS= read -r line; do
  case "$line" in
    *'"method":"handshake"'*) printf '%s\n' '{"version":1}' ;;
    *) printf '%s\n' "$line" ;;
  esac
done
```

Requests are killed after five seconds. stdout must contain only protocol JSON;
write diagnostics to stderr.

单次请求超过五秒会被终止。stdout 只能输出协议 JSON，诊断信息请写入 stderr。

### 4.3 Script environment / 脚本环境变量

The host only exposes approved capabilities:

| Variable | Meaning / 含义 |
| --- | --- |
| `RAYLOAD_PLUGIN_PERMISSIONS` | Comma-separated granted permissions. / 已授予权限，逗号分隔。 |
| `RAYLOAD_PLUGIN_ALLOWED_DOMAINS` | Declared host allow-list. / 声明的域名白名单。 |
| `RAYLOAD_PLUGIN_COOKIES` | JSON map of scoped cookies, only when `cookies` is granted. / 仅在授予 `cookies` 时提供的域名 Cookie JSON。 |

If `cookies` is not granted the variable is absent. Scripts must not print
environment values or cookie data to logs.

未授予 `cookies` 时该变量不存在。脚本不得把环境变量或 Cookie 写入日志。

## 5. Import and testing / 导入与测试

1. Open **Settings → Advanced → Plugins**.
2. Tap **Import plugin** and choose a `.json` manifest or `.sh` script.
3. Review the permission dialog. `cookies`, `read_clipboard` and script plugins
   require the “I understand the risk” checkbox.
4. Tap **Allow and import**.
5. Tap **Test**. The sample plugin downloads the current Rayload APK. Rule
   plugins with `testUrl` perform a permission-checked HTTP request. Script
   plugins must answer the handshake and echo request.

1. 打开 **设置 → 高级 → 插件**。
2. 点击 **导入插件**，选择 `.json` 清单或 `.sh` 脚本。
3. 审查权限。`cookies`、`read_clipboard` 与脚本插件必须勾选“我已知晓风险”。
4. 点击 **允许并导入**。
5. 点击 **测试**。示例插件会下载当前 Rayload APK；带 `testUrl` 的规则插件会执行
   一次经过权限检查的 HTTP 请求；脚本插件必须正确应答握手与回显请求。

## 6. Security checklist / 安全检查清单

- Request only the domains you actually need.
- Never ask for `cookies` when a public endpoint works.
- Do not embed credentials, cookies or personal tokens in a manifest.
- Treat extraction codes as user secrets and never log them.
- Keep `allowedDomains` narrow; avoid broad wildcards.
- Test with a disposable account before using a private cloud drive.
- Remember that script plugins are trusted local code. Rayload can show a
  warning, but it cannot make an untrusted script safe.

- 只申请真正需要的域名。
- 公共接口可用时不要申请 `cookies`。
- 不要在清单中硬编码账号、Cookie 或个人令牌。
- 提取码属于用户秘密，禁止写入日志。
- 保持 `allowedDomains` 尽量窄，避免过宽的通配符。
- 在私有网盘上使用前，先用一次性账号测试。
- 脚本插件属于受信任的本地代码。Rayload 可以警告，但无法让不可信脚本变安全。
