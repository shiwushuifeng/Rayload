#!/usr/bin/env python3
"""Add the randomized BT listen-port toast to every supported locale."""

import json
from pathlib import Path

LOCALES_DIR = Path("src/shared/locales")

COPY = {
    "zh-CN": "BT 监听端口已随机为 {port}",
    "zh-TW": "BT 監聽埠已隨機為 {port}",
    "en-US": "BT listen port randomized to {port}",
}

for path in sorted(LOCALES_DIR.glob("*/messages.json")):
    locale = path.parent.name
    messages = json.loads(path.read_text(encoding="utf-8"))
    messages.setdefault("preferences", {})["bt-port-randomized"] = COPY.get(
        locale, COPY["en-US"]
    )
    path.write_text(json.dumps(messages, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

print(f"Updated {len(list(LOCALES_DIR.glob('*/messages.json')))} locale files.")
