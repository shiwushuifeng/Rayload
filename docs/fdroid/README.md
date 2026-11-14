# F-Droid submission / F-Droid 提交说明

`com.shiwushuifeng.rayload.yml` is the prepared fdroiddata recipe. The build
sets `RAYLOAD_FDROID=1`, so the resulting applicationId is
`com.shiwushuifeng.rayload.fdroid`. fdroiddata requires the metadata filename
to match the applicationId: rename the file to
`com.shiwushuifeng.rayload.fdroid.yml` in the fdroiddata merge request.

`com.shiwushuifeng.rayload.yml` 是已准备好的 fdroiddata 配方。构建时设置
`RAYLOAD_FDROID=1`，因此实际 applicationId 为
`com.shiwushuifeng.rayload.fdroid`。fdroiddata 要求元数据文件名与
applicationId 一致，提交 MR 时请重命名为
`com.shiwushuifeng.rayload.fdroid.yml`。

## Manual steps / 手动步骤

1. Create and push the release tag so `commit: v1.0.4` resolves:

   ```bash
   git tag v1.0.4
   git push origin v1.0.4
   ```

2. Fork `https://gitlab.com/fdroid/fdroiddata` in the GitLab web UI and clone
   your fork, or use `glab repo fork fdroid/fdroiddata --clone` when `glab` is
   installed and authenticated.
3. Create a branch and copy the recipe:

   ```bash
   git checkout -b rayload-1.0.4
   cp /path/to/Rayload/docs/fdroid/com.shiwushuifeng.rayload.yml \
      metadata/com.shiwushuifeng.rayload.fdroid.yml
   ```

4. Verify the recipe with the F-Droid tools:

   ```bash
   fdroid lint com.shiwushuifeng.rayload.fdroid
   fdroid build com.shiwushuifeng.rayload.fdroid:10006
   ```

5. Commit, push and open a merge request against `fdroid/fdroiddata` `master`:

   ```bash
   git add metadata/com.shiwushuifeng.rayload.fdroid.yml
   git commit -m "New app: Rayload 1.0.4"
   git push -u origin rayload-1.0.4
   glab mr create --source-branch rayload-1.0.4 --target-branch master \
     --title "New app: Rayload" --description "Add com.shiwushuifeng.rayload.fdroid 1.0.4"
   ```

   Without `glab`, open the fork in GitLab and use **Merge requests → New merge
   request**, then select the `fdroiddata` `master` target.

6. Wait for the fdroiddata CI build. Fix any toolchain or NDK issue raised by
   the pipeline, then respond to reviewer comments in the MR.

## Notes / 注意事项

- `fastlane/metadata/android/en-US/` in this repository provides the store
  listing, icon, feature graphic and screenshots.
- The F-Droid APK is signed by F-Droid, not by the GitHub release key. Users
  switching from a GitHub build must uninstall it first if the package ID matches.
- Keep `CurrentVersionCode: 10006` in sync with `tauri.android.versionCode` for
  every future release.
