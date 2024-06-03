# Toolkit

Toolkit is a CLI that helps you to initialize your development environment faster.

## Features

🌎 Cross-platform support (macOS, Windows. Linux is coming soon)

✨ Support custom your toolkits and install them with one command

📦 Share your toolkits manifest with your team members

🚀 Built with rust and install toolkits in parallel

## Quick Start

### 1. Download the Toolkit

You can download the toolkit CLI from the [release page](https://github.com/apptools-lab/AppToolkit/releases).

### 2. Extract the Zip

Extract the zip file to your local directory. Then you will get the `toolkit` binary file(macOS) or `toolkit.exe` file(Windows).

### 3. Run the Toolkit

#### Windows

```shell
# cd the directory where the `toolkit` file is located
$ cd .\Downloads

# install the recommended toolkits(for web development)
$ .\toolkit.exe install
# install your custom toolkits by specifying the manifest file path
$ .\toolkit.exe install --manifest <your-manifest-path>
```

**NOTE: Currently, you may need to follow the installtion instructions to install toolkits manually on Windows system.**

#### Macos

```shell
# cd the directory where the `toolkit` file is located.
$ cd ~/Downloads

$ chmod +x toolkit

# install the default toolkits (for web development)
$ ./toolkit install
# install your toolkits by specifying the manifest file path
$ ./toolkit install --manifest <your-manifest-path>
```

## Customization

You can customize your toolkits which to be installed in a json file. Here is a [json schema](./toolkits.schema.json) for you to follow and you can get the hint in the popular IDEs like Visual Studio Code, IntelliJ and so on. For Example:

```json
{
  "$schema": "https://raw.githubusercontent.com/apptools-lab/AppToolkit/feat/cli/toolkits.schema.json",
  "description": "<your toolkits schema description>",
  "author": "<your name or email>",
  "version": "<toolkits manifest version>",
  "toolkits": [
    {
      "name": "Visual Studio Code",
      "description": "<vscode description>",
      "installations": [
        {
            ...
        }
      ]
    }
  ]
}
```

Now AppToolkit supports the following types of tool:

- Windows
  - [x] exe
- MacOS
  - [x] dmg
  - [x] zip
  - [x] shell
- Linux
  - [ ] deb
  - [ ] rpm
  - [ ] shell

If you want to support more types of toolkits, you can submit a PR or issue to us.

You can see [tookits.manifest.json](./toolkits.manifest.json) for reference.

After completion, you can save your toolkit schema in a json file locally or upload it to the remote server(GitHub repo or cloud storage). You can share your toolkit manifest with your team members or friends.

Then you can install your custom toolkits with the following command:

```shell
# windows
$ .\toolkit.exe install --manifest https://the-remote-server/your-toolkits-manifest-path

# macos
$ ./toolkit install --manifest https://the-remote-server/your-toolkits-manifest-path
```

> Toolkit is still in the early stage of development, and we are working hard to improve it. If you have any suggestions or ideas, please feel free to submit an issue or PR.
