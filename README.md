# Toolkit

Toolkit is a CLI that helps you to initialize your development env faster.

## Features

🌎 Cross-platform support (macOS, Windows, Linux)
✨ Custom your tools list and install them with one command
🚀 Built with rust and install tools in parallel

## Quick Start

### 1. Download the Toolkit

You can download the toolkit CLI from the [release page](https://github.com/apptools-lab/AppToolkit/releases).

### 2. Run the Toolkit

#### Windows

```shell
# Install the recommended toolkits(for web development)
$ toolkit.exe install
# Install your toolkits
$ toolkit.exe install --manifest <your-manifest-path>
```

**NOTE: Currently, you may need to follow the installtion instructions to install tools manually on Windows system.**

#### Macos & Linux

```shell
# Install the recommended toolkits(for web development)
$ toolkit install
# Install your toolkits
$ toolkit install --manifest <your-manifest-path>
```

## Customization

You can customize your tools which to be installed in a json file. Here we provide a [json schema](https://raw.githubusercontent.com/apptools-lab/AppToolkit/feat/cli/toolkits.schema.json) for you to follow and you can get the hint in the popular IDEs like Visual Studio Code, IntelliJ and so on. For Example: 

```json
{
  "$schema": "https://raw.githubusercontent.com/apptools-lab/AppToolkit/feat/cli/toolkits.schema.json",
  "description": "<your toolkits schema description>",
  "author": "<your name or email>",
  "version": "<toolkits manifest version>",
  "tools": [
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

If you want to support more types of tools, you can submit a PR or issue to us.

You can see [tookits.manifest.json](https://raw.githubusercontent.com/apptools-lab/AppToolkit/feat/cli/toolkits.manifest.json) for reference.

After completion, you can save your toolkit schema in a json file locally or upload it to the remote server(GitHub repo or cloud storage). You can share your toolkit manifest with your team members or friends.

Then you can install your custom tools with the following command:

```shell
# windows
$ toolkit.exe install --manifest <your-manifest-path>

# macos & linux
$ toolkit install --manifest <your-manifest-path>
```
