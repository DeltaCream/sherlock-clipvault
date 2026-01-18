# Sherlock Clipboard History (with clipvault!)

This is a lightweight program to enable clipboard history support for Sherlock using [clipvault](https://github.com/Rolv-Apneseth/clipvault), which is adapted from a similar program which uses [cliphist](https://github.com/Skxxtz/sherlock-clipboard) instead. Do check them out!


## Usage



```bash
sherlock-clipvault | sherlock | clipvault get | wl-copy
```

This works if your clipboard entries mostly contain text. However, if you have a lot of images in your clipboard and you want to improve how quick it opens/prevent hanging due to a lot of images on your clipboard, you can use this:

```bash
sherlock-clipvault --shrink-thumbnails | sherlock | clipvault get | wl-copy
```

The thumbnails usually are small enough that they're indistinguishable for most cases, so I would recommend the above command instead.

## Differences from sherlock-clipboard
1. This project uses rayon to parallelize retrieval of entries from `clipvault get`.
2. This program also supports the use of a flag (`--shrink-thumbnails`) for downscaling images when displayed within sherlock for performance. The default command ***does not*** downscale images by default, in case people might use the extra resolution that their original images would have, but the option is there if you need your clipboard to open as fast as possible.

## Installation

### Runtime Dependencies
- `clipvault`

### <ins>From Source</ins>

To build sherlock-clipboard from source, follow these steps.<br>
Make sure you have the necessary dependencies installed:

- `rust` - [How to install rust](https://www.rust-lang.org/tools/install)
- `git` - [How to install git](https://github.com/git-guides/install-git)

1. **Clone the repository**:

    ```bash
    cd /tmp
    git clone https://github.com/DeltaCream/sherlock-clipvault.git
    cd sherlock-clipboard
    ```

2. **Build the project**:

    ```bash
    cargo build --release
    ```

3. **Install the binary**:

    After the build completes, install the binary to your system:

    ```bash
    sudo cp target/release/sherlock-clipvault /usr/local/bin/
    ```

4. **(Optional) Delete the build directory**:

    ```bash
    rm -rf /tmp/sherlock
    ```
