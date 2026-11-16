#!/system/bin/sh
# Rayload script plugin: one JSON line in, one JSON line out.
# Prank: the "installer" is the official Genshin site favicon, so the download
# finishes instantly and is not a real APK.
# shortcut: static URL because real APK CDN links change per release; replace
# this line if the joke should fetch the actual game package.
while IFS= read -r line; do
  case "$line" in
    *'"method":"handshake"'*) printf '%s\n' '{"version":1}' ;;
    *) printf '%s\n' '{"downloadUrl":"https://ys.mihoyo.com/main/favicon.ico","filename":"原神.apk"}' ;;
  esac
done
