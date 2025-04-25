# `rustomorin-analyzer` 一輩子寫 Rust

這是一個帶了 [MyGO 梗](https://game.udn.com/game/story/122089/8357672) 的 Rust linter

雖然這是一個 LSP Server ，但由於我只是想要補齊 Clippy 沒有但我覺得很重要的規則，所以並不是 [`rust-analyzer`](https://github.com/rust-lang/rust-analyzer) 的替代品

寫 Rust 程式時請不要只開這個 LSP ，除非你想感受地獄模式的 Rust 程式設計體驗

## 為啥啊？

原本我只是想要寫個能幫我檢查有沒有用到 `unwrap`, `panic!`, `'static`, `unsafe` 和 `?` 的 linter ，研究了一圈發現直接寫新的 LSP Server 比較快

某天聽特級咒物春日影時，突然覺得把邦邦世家的名場面臺詞搬上去也不錯，於是腦子一熱就變成這樣了，分享到 X 上後，沒想到還蠻多人按讚的

<blockquote class="twitter-tweet" data-dnt="true"><p lang="zh" dir="ltr">一輩子寫 Rust <a href="https://t.co/2VWgHvkzL0">pic.twitter.com/2VWgHvkzL0</a></p>&mdash; Ming Chang (@mingchang137) <a href="https://twitter.com/mingchang137/status/1914984493540409550?ref_src=twsrc%5Etfw">April 23, 2025</a></blockquote>

所以其實也沒啥特別的理由，一切都是偶然（笑

## 怎麼編譯

1. [安裝 Rust](https://rustup.rs)
2. 用 Git 把這個專案 Clone 下來
3. 切換到 Clone 下來的目錄後，在終端機輸入 `cargo build --release`
4. 編譯完成後， `target/release` 目錄中的 `rustomorin-analyzer` (*nix) / `rustomorin-analyzer.exe` (Windows) 檔案就是 LSP 本體了

## 怎麼用

### Neovim (0.11.0 以上)

```lua
vim.lsp.config("rustomorin-analyzer", {
    cmd = { "*請將此處改成 rustomorin-analyzer 執行檔的路徑*" },
    root_markers = { "Cargo.toml" },
    filetypes = { "rust" }
});
vim.lsp.enable('rustomorin-analyzer');
```

### 蛤？其他編輯器呢？

由於我早在兩年前就已經決定一輩子只用 Neovim 了，所以我也不知道其他 Code editor 或 IDE 要怎麼註冊 LSP Server ，歡迎發 PR contribute

## 連結

未來我會寫一篇部落格文章分享 LSP 開發的過程，所以先預留這個段落，啊如果你有興趣看看我的其他文章，這邊是傳送門：[https://blog.mingchang.tw](https://blog.mingchang.tw)

## License

rustomorin-analyzer is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
