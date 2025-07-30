# フォルダ・モジュール構成まとめ（2025/07/31時点）

## ディレクトリ構成（例）

```
dot_punk/
├── src/
│   ├── main.rs
│   ├── core.rs
│   ├── core/
│   │   ├── resouce.rs
│   │   ├── save_system.rs
│   │   ├── setting.rs
│   │   └── ui/
│   ├── states/
│   │   ├── main_menu.rs
│   │   ├── in_game.rs
│   │   └── ...
│   ├── game/
│   │   ├── world/
│   │   ├── player/
│   │   ├── ui/
│   │   └── ...
│   └── ...
├── assets/
├── config/
├── tests/
└── design/
```

## モジュール構成例

```rust
// filepath: src/core.rs
pub mod resouce;
pub mod save_system;
pub mod setting;
pub mod ui;
```

- `core/`：共通リソース、セーブ、設定、UIなどの基盤機能
- `states/`：ゲーム状態ごとのロジック（例：メインメニュー、ゲーム中など）
- `game/`：ゲーム固有のロジックやエンティティ
- `assets/`：画像・音声などのアセット
- `config/`：設定ファイル
- `tests/`：テストコード
- `design/`：設計資料やドキュメント

---

### 補足
- 共通的な処理やリソースは `core/` に集約
- 状態ごとの処理は `states/` に分離
- ゲームの拡張要素は `game/` 配下に追加

この構成をベースに、今後の拡張や保守も容