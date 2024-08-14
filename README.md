# BLANE - Lightweight LAN Communication Tool in Rust

For the Chinese version of the description, please refer to [中文版说明](/README_CN.md).

## About This Project

This project was initiated after a CTF offline competition (🚄⛰💧☔). With the challenging global public health situation, many CTF competitions have shifted to online formats. However, our team participated in an offline CTF competition for the first time. Due to our lack of familiarity with the competition format and insufficient preparation, we encountered difficulties in communication and collaboration among team members, especially when spread across different locations. The competition venue had restricted internet access and limited time, which further hindered effective communication, data sharing, and collaboration. As a result, I decided to develop a program that facilitates convenient communication within a local area network. This program can be useful in similar scenarios. This project aims to establish a basic framework, and I will continuously update and improve its functionalities.

+ BLANE is a LAN chat tool developed in Rust, designed for daily communication among devices within a local area network.

+ It utilizes asymmetric encryption algorithms for secure data transmission and supports text communication (both Chinese and English), as well as image and file transfer capabilities.

+ The project aims to provide a convenient, secure, and lightweight chat experience for members of small and medium-sized teams within a local area network.

If you have any ideas or suggestions, please feel free to raise an issue. Your support, attention, and contributions are highly appreciated. This project is in its early development stage, and updates will be made at my own pace.

Your attention and stars are welcomed🥰!

## Features

- Secure communication through asymmetric encryption algorithms
- Text communication supporting both Chinese and English languages
- Image and file transfer or sharing capabilities
- Online status tracking
- Customizable usernames
- etc..

## Build

1. Clone the repository:

   ```bash
   git clone https://github.com/DXHM/BLANE.git
   ```

2. Build the project:

   ```bash
   cd BLANE
   cargo build
   ```

3. Run the server:

   ```bash
   cargo run --bin server
   ```

4. Run the client:

   ```bash
   cargo run --bin client
   ```

## Dependencies

- glib-2.0: Required dependencies for the server and client GUI (based on GTK)
- openssl: Required for encryption algorithms

## Contribution

[<img alt="AShujiao" src="https://avatars.githubusercontent.com/u/69539047?v=4" width="117">](https://github.com/dxhm)


## License
[![gplv3](https://www.gnu.org/graphics/gplv3-or-later.png)](https://www.gnu.org/licenses/gpl-3.0.txt)

## If it useful for you. Please offer me a coffee. ☕
<img src="https://raw.githubusercontent.com/DXHM/DXHM/main/tipcode.jpg" title="tipcode" height="50%" width="50%">

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=DXHM/BLANE&type=Date)](https://star-history.com/#DXHM/BLANE&Date)
