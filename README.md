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
# Install the recommended tools
$ toolkit.exe install
# Install your custom tools
$ toolkit.exe install --config <your-config-path>
```

#### Macos & Linux

```shell
# Install the recommended tools
$ toolkit install
# Install your custom tools
$ toolkit install --config <your-config-path>
```

## Customization

You can customize your tools list by creating a json file. Here we provide a [json schema](https://gist.githubusercontent.com/luhc228/a71577a3a10688469f0e8e3a44b99cc5/raw/c1fb0541a183adf2d2d54c2f1da8b49a9f81c8e1/toolkit.installation-info.schema.json) for you to follow and you can get the hint in some IDE just like Visual Studio Code.

```json
{
   "$schema": "https://gist.githubusercontent.com/luhc228/a71577a3a10688469f0e8e3a44b99cc5/raw/c1fb0541a183adf2d2d54c2f1da8b49a9f81c8e1/toolkit.installation-info.schema.json",
   "tools": [
        {
            "name": "Visual Studio Code",
            "description": "<description>",
            "installations": [
                {
                    ...
                }
            ]
        }
   ],
   "installType": "parallel"
}
```

After completion, you can save your tools list in a json file locally or upload to the remote server(GitHub Gist or other cloud storage). You can share your tools list with your team members or friends.

Then you can install your custom tools with the following command:

```shell
# windows
$ toolkit.exe install --config <your-config-path>

# macos & linux
$ toolkit install --config <your-config-path>
```