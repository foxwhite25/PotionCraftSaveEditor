# Potion Craft 存档编辑/解码器

这游戏的存档底层是 json ，套了一层 xor 加密之后再使用 base64 编码。
这个工具可以解码游戏的存档，然后你可以用任何文本编辑器编辑它。
或者你可以写一个自动修改存档的脚本，然后再用这个工具重新编码。

## 使用方法
自动化目前默认行为为将所有药水材料需求设置为空，只需要基底就可以无中生有。
如果你想要修改其他内容，可以自行修改源码。也可以解码之后手动修改，然后再编码。

这游戏的存档位置在 `C:\Users\你的用户名\AppData\LocalLow\niceplay games\Potion Craft` 。

### 1. 解码
```bash
cargo run --release -- decode <input>
```
解码后的文件是两个 json 文件，一个是 `meta.json` ，一个是 `progress.json` 。

`meta.json` 里面是游戏的一些让你在存档界面看到的信息，比如你的名字、游戏时间、游戏版本等等。

`progress.json` 里面是游戏的进度，比如你的金钱、药水材料、药水配方等等。

### 2. 编码
```bash
cargo run --release -- encode <output>
```
运行的时候需要 `meta.json` 和 `progress.json` 两个文件在同一个目录下。

### 3. 自动化
```bash 
cargo run --release -- edit <input> <output>
```
输出是一个临时文件，你需要手动将它覆盖到游戏存档中。