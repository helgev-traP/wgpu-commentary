# wgpu/WebGPU概説

- 立ち位置
  - OpenGL系・DX11に対してのVulkan・DX12・Metalと同じ立ち位置
  - 現代的なマルチスレッドレンダリング対応の低レベルAPI
  - Vulkan・DX12・Metalを薄いラッパーで抽象化し、プラットフォームのことを考えなくても良いようにするAPI
  - Vulkan・DX12・Metal固有の機能には直接触れない場合がある。
  - WebGPU層でのCPU側のバリデーションが存在する
  - しかし、GPU駆動レンダリングにも対応しており、GPU側で処理を行ってしまえばWebGPU層は気にしなくてよい。それでもパフォーマンスを上げる必要があるのならVulkanかDX12を使おう。
- WebGLとの違い
  - OpenGL系が現代で辛いこと
    - マルチスレッドではない
    - 低レベルではない
  - WebGPU
    - もちろんマルチスレッド
    - Vulkan・DX12・Metalの構造が似ているのでラッパーが薄い。
    - 現代的な機能が使える
      - Indirect描画
      - レイトレーシング
- ネイティブでしかできないこと
  - Push Constants
  - MultiView
- できないこと
  - 高度なGPU駆動レンダリング
    - ワークグラフ

# winit概説

- winitとは
  - ウィンドウィングライブラリ
  - あらゆるプラットフォームでウィンドウを作成し、グラフィックライブラリに描画ターゲットを渡すライブラリ
    - Windows, Linux, Mac, Web, Android, iOS
  - ブラウザでもWasmで動く
  - 入力も共通化して渡してくれる
- winitの使い方
  - 空のウィンドウの出し方
  - ApplicationHandler
    - resume
    - window event
    - new events
