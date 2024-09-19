# 🌈 `gradcat` 🐈

A lolcat clone.

<!-- TODO: Screenshot Here -->

## 📘 Usage

Give it a list of files to cat out or pipe it in through `stdin`.

```sh
gradcat ./src/main.rs
```

```sh
git log | gradcat
```

```
Usage: gradcat [FILES...] [OPTIONS]

Options: 
  --mode <mode>	Set the gradient mode (rainbow, linear)

  --frequency <#color>	Rainbow gradient's sine-wave frequency (Higher values cause faster change in the pattern)
  --spread <#color>	Rainbow gradient's spread value

  --start-color <#color>	Set the start color for the linear gradient
  --end-color <#color>	Set the end color for the linear gradient


  --help	Display this help message
  --version	Display the version number
```

---

## 📄 License

This project is licensed under the [MIT License](./LICENSE). Read the [LICENSE](./LICENSE) file for more details.
