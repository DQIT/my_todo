// 拉取 GitHub 最新 Release，填充版本号并把下载按钮指向安装包资源。
(function () {
  "use strict";

  var REPO = "DQIT/my_todo";
  var verEl = document.getElementById("dl-version");
  var btnEl = document.getElementById("dl-button");

  // 浮动动画（尊重 reduce-motion 由 CSS 控制）
  var panel = document.querySelector(".layer-panel");
  if (panel) panel.classList.add("float");

  function setText(t) { if (verEl) verEl.textContent = t; }

  fetch("https://api.github.com/repos/" + REPO + "/releases/latest", {
    headers: { Accept: "application/vnd.github+json" },
  })
    .then(function (r) {
      if (!r.ok) throw new Error("HTTP " + r.status);
      return r.json();
    })
    .then(function (rel) {
      var tag = rel.tag_name || "";
      // 在资源里找 Windows 安装包（.exe / .msi）
      var asset = (rel.assets || []).filter(function (a) {
        return /\.(exe|msi)$/i.test(a.name);
      })[0];

      if (verEl) {
        verEl.innerHTML = tag
          ? "最新版本 <b>" + tag + "</b> · 免费开源"
          : "免费开源 · 永久免费";
      }
      if (asset && btnEl) {
        btnEl.href = asset.browser_download_url;
      }
    })
    .catch(function () {
      // 接口失败（无 Release / 限流）时退化为 releases 页面入口
      setText("免费开源 · 永久免费");
    });
})();
