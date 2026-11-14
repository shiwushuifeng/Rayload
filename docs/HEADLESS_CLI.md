# Headless CLI / 无界面 CLI

`ray` (also installed as `rayload`, and built as `rayload-cli`) is Rayload's
terminal download console. It runs Aria2 plus the LAN API without the Android
UI, and adds task control, pairing management, daemon mode and systemd support.

`ray`（同时提供 `rayload` 别名）是 Rayload 的终端下载控制台。它在无界面环境
中运行 Aria2 与局域网 API，并提供任务控制、配对管理、守护进程和 systemd
支持。

## Build / 构建

```bash
cargo build --release --manifest-path src-tauri/Cargo.toml --features cli --bin ray --bin rayload
```

The binaries are written to `src-tauri/target/release/ray` and
`src-tauri/target/release/rayload`. The older `rayload-cli` target remains as a
compatibility alias. Aria2 must be available on `PATH` as `aria2-next` by
default; override it with `download.engine_bin`, `--engine-bin` or
`RAYLOAD_ARIA2_BIN`.

## Quick commands / 快速命令

```bash
ray init                 # interactive config.yml
ray start                # foreground engine + LAN API
ray start -d             # daemon; log at <data-dir>/rayload.log
ray -s                   # tree status (alias: ray status)
ray -d list              # paired-device tree
ray install              # install and enable the systemd unit
```

Typical first run on a NAS:

```bash
ray init
ray start -d
ray -s
ray -d list
sudo ray install
```

`ray init` asks for the download directory, device name, LAN port, admin
credentials, manual-approval policy and the main Aria2 tuning values, then
writes a `0600` YAML file. `ray install` must run as root and registers
`rayload.service` with `systemctl enable --now`.

`ray init` 会询问下载目录、设备名、局域网端口、管理账号密码、手动确认策略与
主要 Aria2 参数，并写入权限为 `0600` 的 YAML；`ray install` 需要 root，
会注册并立即启用 `rayload.service`。

## Configuration / 配置

Default file: `~/.config/rayload/config.yml` (honors `XDG_CONFIG_HOME`).
Use `--config PATH` or `RAYLOAD_CONFIG` to point somewhere else.

```yaml
device:
  name: "My-NAS"

lan:
  enabled: true
  port: 6767
  username: "admin"
  password: "your_password"
  require_manual_confirm: false

download:
  dir: "/data/downloads"
  engine_bin: "aria2-next"
  rpc_port: 6800
  aria2_options:            # raw Aria2 options
    max-concurrent-downloads: 5
    max-connection-per-server: 16
    split: 16
    # any other aria2.conf key can be added here
```

Priority: **command-line flags > environment variables > config.yml > defaults**.
Managed keys (`dir`, RPC listen settings, session paths) cannot be overridden by
`aria2_options`; all other engine options are passed through.

优先级：**命令行参数 > 环境变量 > config.yml > 默认值**。`dir`、RPC 监听、
会话文件等受管选项不会被 `aria2_options` 覆盖。

### Interactive setup / 交互式初始化

```bash
ray init
ray init --force     # overwrite an existing config
```

`ray init` asks for the download directory, device name, LAN settings, password,
manual-approval policy and the main Aria2 tuning values, then writes a `0600`
YAML file.

### Inspect and edit / 查看与修改

```bash
ray config                  # alias: ray -c
ray config show             # effective config, including overrides
ray config path             # config file location
ray config get lan.port
ray config set lan.port 6768
ray config set lan.require_manual_confirm true
ray config set download.aria2_options.split 32
ray auth                    # alias: ray -a, change username/password
```

`ray auth` writes the new credentials to YAML. Restart the service for the
running process to pick them up.

## Commands / 命令

| Command | Description |
| --- | --- |
| `ray init` | Interactive `config.yml` generation |
| `ray start` | Run Aria2 + LAN API in the foreground |
| `ray start -d` | Run detached; logs go to `<data-dir>/rayload.log` |
| `ray stop` | Stop the background process |
| `ray security-check` | Audit LAN credentials, ports, download path and user |
| `ray status`, `ray -s` | Tree status: engine, RPC, LAN, speed, devices |
| `ray status --json` | Machine-readable status |
| `ray add <url>` | Queue a URL (`--out NAME`, `--json`) |
| `ray list`, `ray ls` | Download table: GID, name, progress, speed, status |
| `ray list --json` | Machine-readable task list |
| `ray pause <gid>` | Pause a task |
| `ray resume <gid>` | Resume a paused task |
| `ray rm <gid>` | Remove the task from Aria2; files stay on disk |
| `ray log -n 40` | Print the daemon log tail |
| `ray log -f` | Follow the daemon log like `tail -f` |
| `ray device`, `ray -d` | Device commands (below) |
| `ray plugin list` | List JSON plugin manifests |
| `ray install` | Install and enable the systemd unit |

### Security audit / 安全体检

```bash
ray security-check
ray security-check --json
```

The report checks the LAN listener, the loopback-only Aria2 RPC port, LAN
credentials, download directory and whether the process runs as root. At least
one `FAIL` produces exit code `1`, so it can gate startup scripts or CI.

### Status tree / 状态树

```text
Rayload
├─ Device  My-NAS
├─ Engine  running (pid 1234)
├─ RPC  127.0.0.1:6800
├─ LAN  http://0.0.0.0:6767
├─ Speed  1.2 MiB/s
├─ Active  2
└─ Devices  3 paired (2 online)
```

Online devices are green, offline devices are red, and stopped services are red.
Colours are disabled automatically when output is piped, or when `NO_COLOR` is
set.

### Task console / 任务控制

`ray add` and the task commands talk to Aria2 directly on
`127.0.0.1:<download.rpc_port>` (default `6800`), so the daemon must be running.
`ray rm` removes the task and its Aria2 result record; downloaded files are
intentionally kept.

```bash
ray add https://example.com/file.iso
ray add https://example.com/file.iso --out custom.iso --json
ray list
ray pause 2089b05ecca3d829
ray resume 2089b05ecca3d829
ray rm 2089b05ecca3d829
```

### Device management / 设备管理

```bash
ray -d list                 # tree view, ids, online state, last seen
ray -d list --json
ray -d revoke <id>
ray -d pending              # pairing requests waiting for approval
ray -d pending --json
ray -d allow <id>
ray -d reject <id>
```

`ray -d list` prints a coloured tree; online peers are green and offline peers
are red. JSON output is available with `ray -d list --json`:

`ray -d list` 输出彩色树状图，在线为绿色、离线为红色；`--json` 可输出结构化数据：

```text
Paired devices
├─ Living-room tablet  ● online  id 3f1c9a2e
│  └─ last seen 2 minutes ago
└─ Office NAS          ○ offline  id 8b77d104
   └─ last seen 3 hours ago
```

When `lan.require_manual_confirm: true`:

- Foreground `ray start` shows a `y/N` prompt in the terminal.
- Background/systemd mode leaves the request pending until
  `ray -d allow <id>` (or `ray -d reject <id>`) runs against the local API.

When `false`, a valid admin password pairs immediately.

### Logs / 日志

```bash
ray log -n 100
ray log -f
```

The log lives at `<data-dir>/rayload.log`. The data directory defaults to
`$XDG_DATA_HOME/rayload-cli` and can be changed with `--data-dir` or
`RAYLOAD_DATA_DIR`.

### Plugins / 插件

```bash
ray plugin list
ray plugin list --dir ./plugins
ray plugin list --json
```

Without `--dir`, `./plugins` and `<data-dir>/plugins` are scanned for direct
`*.json` children. The command reads manifest metadata only; it does not execute
plugins.

## JSON output / JSON 输出

`ray status --json`, `ray list --json`, `ray security-check --json`, `ray -d list --json`,
`ray -d pending --json`, `ray plugin list --json` and `ray add --json` print a
single JSON document and no colours or progress text. This makes them safe to
feed to `jq`, Python or a monitoring script.

```bash
ray status --json | jq .speed
ray list --json | jq '.tasks[] | select(.status == "active")'
```

## Environment variables / 环境变量

| Variable | Purpose | Default |
| --- | --- | --- |
| `RAYLOAD_CONFIG` | Config file path | `~/.config/rayload/config.yml` |
| `RAYLOAD_DATA_DIR` | Engine state, LAN identity, logs, PID | `$XDG_DATA_HOME/rayload-cli` |
| `RAYLOAD_ARIA2_BIN` | Aria2 binary | `aria2-next` |
| `RAYLOAD_DOWNLOAD_DIR` | Download root | `<cwd>/downloads` |
| `RAYLOAD_DEVICE_NAME` | Device name on the LAN | host name |
| `RAYLOAD_LAN_PORT` | LAN API port | `6767` |
| `RAYLOAD_RPC_PORT` | Aria2 RPC port | `6800` |
| `RAYLOAD_LAN_USERNAME` | LAN admin user | none |
| `RAYLOAD_LAN_PASSWORD` | LAN admin password | none |
| `RAYLOAD_REQUIRE_MANUAL_CONFIRM` | Require pairing approval | `false` |

## Systemd / systemd

```bash
sudo ray install
systemctl status rayload
journalctl -u rayload -f
```

The command must run as root. It copies the current binary to
`/usr/local/bin/ray`, creates `/usr/local/bin/rayload`, writes
`/etc/systemd/system/rayload.service`, then runs `systemctl daemon-reload` and
`systemctl enable --now rayload.service`. The service runs as `SUDO_USER` (or
`root` when there is no sudo user) and reads that user's config file. Run
`ray init` as the service user before installing.

## Docker / 容器

```bash
docker build -t rayload-cli .
docker run --rm -p 6767:6767 \
  -e RAYLOAD_LAN_USERNAME=admin \
  -e RAYLOAD_LAN_PASSWORD='change-me' \
  -v "$PWD/downloads:/downloads" \
  -v rayload-data:/data \
  rayload-cli
```

The image runs as UID 10001. Mount a config file instead of using environment
variables when you want `aria2_options`:

```bash
docker run --rm -p 6767:6767 \
  -v "$PWD/config.yml:/config/config.yml:ro" \
  -v "$PWD/downloads:/downloads" \
  -v rayload-data:/data \
  rayload-cli --config /config/config.yml start
```

Notes:

- Download and data volumes must be writable by UID 10001.
- Port `6800` stays on container loopback; only publish `6767`.
- Use a firewall or reverse proxy if the LAN API crosses untrusted networks.
