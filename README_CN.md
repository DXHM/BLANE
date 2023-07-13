
# BLANE - 基于RUST的轻量化局域网通信工具


## 关于本项目

此项目立项想法是在某次CTF线下赛(🚄⛰💧☔)后产生的，全球范围内的公共卫生环境的形势严峻，此前参加过的很多CTF以线上赛为主，开放后也是和队友们都第一次参加线下CTF比赛。我的团队由于对赛制缺乏了解和提前准备，对于客场分布情况及影响沟通效率都存在问题。比赛现场禁用互联网并且时间紧迫，也会给团队间的沟通交流，资料共享，协同合作产生很多障碍，耽误很多时间。由此我产生了自己编写一个应用于局域网内实现便捷通信的程序，可以在这种类似的场景下派上用场，先立项搭建个大体框架，不定时更新并将一些功能不断完善进去。

+ 本项目为基于Rust编写的一个局域网聊天工具，用于局域网设备间的日常通讯。

+ 采用非对称加密算法进行数据加密传输，可以进行文字通信（支持中/英），并具备图片和文件传输功能。

+ 该项目旨在为中小型团队成员在局域网内提供一个方便快捷，安全且轻量化的聊天体验。

各位师傅如果什么想法欢迎issue，求关注求star！感谢您的支持，欢迎各位大佬贡献代码，本项目仍处于早期开发阶段，佛系更新。

欢迎催更，下次一定😋！


## 功能特点

- 通过非对称加密算法实现安全通信
- 支持中英文文字通信
- 图片和文件传输或共享
- 在线状态跟踪
- 可自定义用户名
- etc..

## 构建

1. 克隆仓库：

   ```bash
   git clone https://github.com/DXHM/BLANE.git
   ```

2. 构建项目：

   ```bash
   cd BLANE
   cargo build
   ```

3. 运行服务器：

   ```bash
   cargo run --bin server
   ```

4. 运行客户端：

   ```bash
   cargo run --bin client
   ```


## 依赖项

- glib-2.0：服务器和客户端 GUI 所需的依赖项（基于 GTK 的界面）
- openssl：用于加密算法


## 贡献者

[<img alt="AShujiao" src="https://avatars.githubusercontent.com/u/69539047?v=4" width="117">](https://github.com/dxhm)

## 许可证

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=DXHM/BLANE&type=Date)](https://star-history.com/#DXHM/BLANE&Date)


