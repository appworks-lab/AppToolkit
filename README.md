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

#### Macos & Linux

```shell
# Install the recommended toolkits(for web development)
$ toolkit install
# Install your toolkits
$ toolkit install --manifest <your-manifest-path>
```

## Customization

You can customize your tools list by creating a json file. Here we provide a [json schema](https://raw.githubusercontent.com/apptools-lab/AppToolkit/feat/cli/toolkits.schema.json) for you to follow and you can get the hint in some IDE just like Visual Studio Code.

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

You can see [tookits.manifest.json](https://raw.githubusercontent.com/apptools-lab/AppToolkit/feat/cli/toolkits.manifest.json) for reference.

After completion, you can save your tools list in a json file locally or upload to the remote server(GitHub repo or other cloud storage). You can share your toolkit manifest with your team members or friends.

Then you can install your custom tools with the following command:

```shell
# windows
$ toolkit.exe install --manifest <your-manifest-path>

# macos & linux
$ toolkit install --manifest <your-manifest-path>
```
