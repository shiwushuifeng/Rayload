# FAQ / 常见问题

## Why does Rayload need to stay alive in the background? / 为什么需要在后台保活？

Downloads run inside a foreground service. Aggressive vendor power management can
freeze or kill that service, so long tasks stall. Allow notifications, exclude
Rayload from battery optimization, and keep the notification visible. Keeping the
app open or using keep-awake mode while charging also helps.

下载由前台服务执行。部分厂商的省电策略会冻结或杀掉该服务，导致任务中断。
请允许通知、授予电池优化豁免并保留常驻通知；充电时保持应用前台或开启不息屏
挂机模式也更稳定。

## Why are downloads slow? / 下载速度慢怎么办？

- Raise `max-connection-per-server` and `split` in engine settings (defaults are conservative).
- Check the global or scheduled speed limit; remove it temporarily.
- BitTorrent speed depends on peers and seeding; try another tracker or wait for peers.
- A slow proxy, VPN or DNS resolver can bottleneck HTTP downloads.
- Make sure the storage has enough free space and is not throttling writes.

请检查全局/定时限速，适当提高单服务器连接数与分片数；BT 速度取决于做种与
Peer，网络代理、VPN、DNS 与存储写入速度也可能成为瓶颈。

## Pairing fails or the device is not discovered / 配对失败或扫描不到设备

- Both devices must be on the same subnet; guest Wi-Fi and AP isolation block discovery.
- Grant **local network access** on Android 17 for both devices.
- Confirm the admin username and password in **LAN devices → This device**.
- Compare the pairing fingerprint with the peer's local fingerprint.
- Make sure port `6767` is not used by another service.

App 内需要双方确认配对请求；CLI 部署若开启 `require_manual_confirm`，请在终端
输入 `y` 或执行 `ray -d allow <id>`。详见 [LAN 指南](LAN_GUIDE.md)。

## Permissions were denied / 权限被拒绝了怎么办？

- **All files access** is required to write to the public download folder.
- **Notifications** are required for progress and pause/resume actions.
- **Camera** is only needed to scan pairing QR codes.
- **Local network** is required on Android 17 for discovery and pairing.

Open system settings → Apps → Rayload → Permissions, grant the permission, then
reopen the app. Some ROMs also require disabling "restrict background activity".

请在系统设置 → 应用 → Rayload → 权限中重新授权，然后重启应用；部分系统还需
关闭“限制后台活动”。

## Cloud-drive or plugin links fail to parse / 网盘链接解析失败

1. Enable the matching plugin and confirm it is allowed to run.
2. Add the site to the plugin's `allowedDomains` network whitelist.
3. Provide the account cookie or password variable the rule expects.
4. Check that `{{password}}` is used only for share-password fields, not headers.
5. Update the plugin; providers change their page structure regularly.

See [Plugin development guide](PLUGIN_DEVELOPMENT_GUIDE.md) for the JSON schema,
header rewriting and permission model.

请启用对应插件并确认其网络白名单包含目标域名；按规则填写 Cookie 或分享密码。
`{{password}}` 只用于分享密码字段。网盘页面结构变化时需更新插件规则。

## The browser extension cannot connect to RPC / 浏览器扩展连不上 RPC

- Use Firefox for Android or a Chromium build that supports extension sideloading.
- Address must be `http://127.0.0.1:6800/jsonrpc`.
- Copy the current secret from **Settings → Advanced → RPC Secret**.
- The RPC endpoint is loopback-only by design and is not reachable from other devices.

## "App not installed" when updating / 更新时提示“应用未安装”

Release, debug and F-Droid builds use different signing identities or package
names. Uninstall the old build before installing a different variant. Uninstalling
does not delete files already downloaded to the public folder.

## CLI daemon will not start / CLI 守护进程起不来

Check that `aria2-next` is on `PATH` or set `download.engine_bin`, then inspect the
log:

```bash
ray status
ray log -n 100
ray security-check
```

See [Headless CLI manual](HEADLESS_CLI.md) for systemd and Docker deployment.
