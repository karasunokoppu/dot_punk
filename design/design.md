## フォルダ及びファイルの分け方方針（改訂案）

1. **エンティティ単位でフォルダを分割**  
   例：`player`, `npc`, `map` など。各エンティティごとに `components`, `controller` などのサブモジュールを配置。
2. **Pluginごとにディレクトリを作成し、ゲームロジックを管理**  
   各エンティティや機能ごとにPluginを作成し、`core/plugin.rs` などで一括登録。

---

## リファクタリングTODO

- [ ] Pluginの登録関係の見直し
- [ ] フォルダ構成を再構築（下記案を参考）

---

## 課題・顕在化している問題点

- Pluginの読み込み順によってリソースの初期化タイミングが変わるため、依存関係に注意。

---

## 推奨フォルダ構成案

```
dot_punk/
├── main.rs                # エントリーポイント
├── core/                  # 共通処理・リソース・プラグイン
│   ├── plugin.rs
│   ├── component.rs       #共通で使うコンポーネント
│   └── ...
├── states/                # ゲーム状態管理
│   ├── mod.rs
│   ├── splash.rs
│   ├── main_menu.rs
│   └── in_game.rs
├── game/                  # ゲームロジック
│   ├── player/
│   │   ├── mod.rs
│   │   ├── components.rs
│   │   └── controller.rs
│   ├── npc/
│   │   └── ...
│   ├── map/
│   │   ├── mod.rs
│   │   ├── components.rs
│   │   └── teleport_node.rs
│   ├── stage/
│   │   ├── mod.rs
│   │   └── stage001.rs
│   └── ui/
│       ├── mod.rs
│       └── ...
├── assets/                # 画像・音声などのアセット
├── config/                # 設定ファイル
└── tests/                 # テストコード
```

### 補足
- `core/`：全体で使うリソースやプラグインなどの共通処理
- `states/`：状態ごとにファイルを分割し、状態遷移もここで管理
- `game/`：エンティティやマップ、UIなどゲーム固有のロジック
- `assets/`：画像・音声・マップデータなど
- `config/`：設定ファイル（YAML, TOML等）
- `tests/`：ユニットテストや統合テスト

---

この構成案をもとに、実際のディレクトリやファイルの作成も自動化できます。  
ご希望があれば、`mkdir` コマンド例や `mod.rs` の雛形もご提案可能