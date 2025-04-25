vim.lsp.config("rustomorin-analyzer", {
    cmd = { "./target/release/rustomorin-analyzer" },
    root_markers = { "Cargo.toml" },
    filetypes = { "rust" }
});
vim.lsp.enable('rustomorin-analyzer');
