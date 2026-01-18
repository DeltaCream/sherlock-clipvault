# Usage

sherlock-clipvault's bash command:

```bash
sherlock-clipvault | sherlock | clipvault get | wl-copy
```

is adapted from [sherlock-clipboard](https://github.com/Skxxtz/sherlock-clipboard)'s own usage command:

```bash
sherlock-clp | sherlock | cliphist-decode | wl-copy
```

which simply uses `clipvault get` instead of `cliphist-decode`. sherlock-clipvault's bash command pipeline also is based from this [cliphist pipeline](https://github.com/sentriz/cliphist?tab=readme-ov-file#select-an-old-item):

```bash
cliphist list | dmenu | cliphist decode | wl-copy
```
