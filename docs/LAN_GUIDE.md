# LAN Networking Guide / 局域网互联指南

Rayload can pair two devices on the same network and push downloads between
them. Pairing is mutual: both sides store the other device and can send tasks or
view live statistics in either direction.

Rayload 可以在同一局域网内配对两台设备，并双向推送下载任务。配对是对等的：
双方都会保存对方设备，任意一方都能推送任务和查看实时统计。

## Requirements / 前提条件

- Both devices run Rayload on the same LAN and subnet.
- The LAN API port `6767` is reachable; guest Wi-Fi or AP isolation must be off.
- On Android 17, grant **local network access** to both devices.
- Camera permission is only needed for QR-code pairing.

## 1. Prepare this device / 准备本机

Open **LAN devices → This device** and set:

- **Device name** — shown to the other device during pairing.
- **Admin username and password** — the other device uses these credentials.

Use a unique password. The LAN API is meant for a trusted home or office network,
not for exposure to the internet.

打开 **局域网设备 → 本机**，设置 **本机名称** 与 **管理账号密码**。对方设备使用
这组账号密码配对。请使用独立密码，不要把端口直接暴露到公网。

## 2. Pair / 配对

Choose one method on the device that initiates pairing:

1. **Scan QR code** — tap **扫码添加** and scan the QR code shown on the other device.
2. **Scan LAN** — tap **扫描局域网** and pick a discovered device.
3. **Manual** — enter the peer IP address or hostname and the admin password.

The receiving device shows a **pairing request** with the initiator's device name.
Review it, then approve or reject. If you did not start the pairing, reject it.

发起端可选择 **扫码添加**、**扫描局域网** 或手动输入 IP 与密码。接收端会弹出
**配对请求**，显示发起方设备名；确认无误后允许，否则拒绝。

### Verify the fingerprint / 核对指纹

Before trusting a device, compare the fingerprint shown in the confirmation
dialog with the **local fingerprint** on the other device (LAN devices → This
device). A matching fingerprint confirms both sides hold the same key.

信任前，请将确认弹窗中的指纹与对方设备 **本机指纹** 核对一致。

## 3. Use paired devices / 使用已配对设备

- **Push tasks** — in the new-task dialog choose the target device. The task runs
  on that device and appears in its task list.
- **Remote tasks** — the task list aggregates tasks from paired devices; pause,
  resume and delete work remotely while the peer is online.
- **Statistics** — the statistics page has **This device** and **All devices**
  tabs with per-device speed and active task count.
- **Notes** — edit a paired device to add a remark, for example "living-room tablet".

Remote advanced options (proxy, cookies and similar) are hidden when the target
is another device; configure those on the target itself.

## 4. Manual approval on the CLI / CLI 手动确认

For NAS or server deployments set `lan.require_manual_confirm: true` in
`config.yml`. A foreground `ray start` prompts `y/N`; a background daemon keeps
the request pending until you run:

```bash
ray -d pending
ray -d allow <id>
ray -d reject <id>
```

With `false`, a correct admin password pairs immediately. See
[HEADLESS_CLI.md](HEADLESS_CLI.md) for the full command reference.

## 5. Remove a device / 移除设备

Tap **Remove** on the paired device, or run `ray -d revoke <id>` on the CLI side.
Both sides clear the stored peer state; the other device must pair again before
sending tasks.

## Troubleshooting / 故障排查

- Not discovered: confirm both devices are on the same subnet and AP isolation is off.
- Pairing rejected: check the admin password and compare fingerprints.
- "Local network" permission denied: grant it in system settings, then reopen Rayload.
- Port conflict: stop the other service using `6767`, or change `lan.port` on both sides.

More answers: [FAQ](FAQ.md).

---

## 简体中文

### 前提条件

- 两台设备运行 Rayload，且处于同一局域网与网段。
- 局域网 API 端口 `6767` 可访问；访客 Wi-Fi 或 AP 隔离必须关闭。
- Android 17 需要为两台设备授予 **局域网访问权限**。
- 仅扫码配对需要相机权限。

### 1. 准备本机

打开 **局域网设备 → 本机**，设置 **本机名称** 与 **管理账号密码**。对方设备
使用这组账号密码配对。请使用独立密码，不要把端口直接暴露到公网。

### 2. 配对

发起端可选择：

1. **扫码添加**：扫描对方设备显示的配对二维码。
2. **扫描局域网**：从发现列表中选择设备。
3. **手动添加**：输入对方 IP 或主机名与密码。

接收端会弹出 **配对请求**，显示发起方设备名。确认无误后允许，否则拒绝；
若不是你刚刚发起的配对，请直接拒绝。

信任前，请将弹窗中的指纹与对方设备的 **本机指纹** 核对一致。

### 3. 使用已配对设备

- **推送任务**：新建任务时选择目标设备，任务在对方设备执行并出现在其任务列表。
- **远程任务**：任务列表汇总已配对设备的任务，对方在线时可远程暂停、继续与删除。
- **数据统计**：统计页提供 **本机** 与 **全部设备** 两个 Tab，显示各设备速度与活动任务数。
- **备注**：编辑已配对设备可添加备注，例如“客厅平板”。

目标为其他设备时，代理、Cookie 等高级选项会隐藏；请在目标设备上单独配置。

### 4. CLI 手动确认

NAS 或服务器部署可在 `config.yml` 设置 `lan.require_manual_confirm: true`。
前台 `ray start` 会弹出 `y/N` 确认；后台守护进程会挂起请求，等待：

```bash
ray -d pending
ray -d allow <id>
ray -d reject <id>
```

设为 `false` 时，密码正确即直接配对。完整命令见
[HEADLESS_CLI.md](HEADLESS_CLI.md)。

### 5. 移除设备

在已配对设备上点 **移除**，或在 CLI 执行 `ray -d revoke <id>`。双方会清理
保存的对端状态，之后需要重新配对才能推送任务。

### 故障排查

- 扫描不到：确认两台设备在同一网段，且未开启 AP 隔离。
- 配对被拒：检查管理密码，并核对双方指纹。
- 局域网权限被拒：在系统设置中授权后重新打开 Rayload。
- 端口冲突：停止占用 `6767` 的服务，或在两端修改 `lan.port`。

更多问题见 [常见问题](FAQ.md)。
