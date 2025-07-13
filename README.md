# raindrop-mcp-server
A Model Context Protocol (MCP) server implementation for [Raindrop.io](https://raindrop.io), exposing its API as a set of tools for AI agents and automated workflows.

![Rust 2024](https://img.shields.io/badge/Rust-2024-orange)
[![CI](https://github.com/yamahigashi/raindrop-mcp-server-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/yamahigashi/raindrop-mcp-server-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


---

## Table of Contents
1. [Quick Start](#quick-start)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Usage](#usage)
5. [Requirements](#requirements)
6. [Troubleshooting](#troubleshooting)
7. [Contributing](#contributing)
8. [License](#license)

---

## Quick Start

1. **Download** the latest binary from [GitHub Releases](https://github.com/yamahigashi/raindrop-mcp-server-rs/releases/tag/v0.1.0)
2. **Get API Token** from [Raindrop.io Settings → Integrations](https://raindrop.io/settings/integrations)
3. **Configure Claude Desktop** (`claude_desktop_config.json`):
   ```json
   {
     "mcpServers": {
       "raindrop": {
         "command": "/usr/local/bin/raindrop-mcp-server",
         "env": {
           "RAINDROP_ACCESS_TOKEN": "your_token_here"
         }
       }
     }
   }
   ```
4. **Start using**: Ask Claude to "Show me all my Raindrop collections"

## Installation

### Pre-built Binaries (Recommended)

Download from [GitHub Releases](https://github.com/yamahigashi/raindrop-mcp-server-rs/releases/tag/v0.1.0):

**Linux:**
```bash
wget https://github.com/yamahigashi/raindrop-mcp-server-rs/releases/download/v0.1.0/raindrop-mcp-server-v0.1.0-linux.tar.gz
tar -xzf raindrop-mcp-server-v0.1.0-linux.tar.gz
chmod +x raindrop-mcp-server-linux
sudo mv raindrop-mcp-server-linux /usr/local/bin/raindrop-mcp-server
```

**Windows:**
```powershell
Invoke-WebRequest -Uri "https://github.com/yamahigashi/raindrop-mcp-server-rs/releases/download/v0.1.0/raindrop-mcp-server-v0.1.0-windows.zip" -OutFile "raindrop-mcp-server.zip"
Expand-Archive -Path "raindrop-mcp-server.zip" -DestinationPath "."
# Move raindrop-mcp-server-windows.exe to a directory in your PATH
```

### From Source

```bash
git clone https://github.com/yamahigashi/raindrop-mcp-server-rs.git
cd raindrop-mcp-server-rs
cargo build --release
# Binary: ./target/release/raindrop-mcp-server
```

## Configuration

### 1. Raindrop.io API Token

1. Log in to [Raindrop.io](https://raindrop.io)
2. Go to **Settings** → **Integrations**
3. Create a new app or select an existing one
4. Generate an access token

### 2. MCP Client Setup

**For Claude Desktop**, edit `claude_desktop_config.json`:
```json
{
  "mcpServers": {
    "raindrop": {
      "command": "/usr/local/bin/raindrop-mcp-server",
      "env": {
        "RAINDROP_ACCESS_TOKEN": "your_raindrop_access_token_here"
      }
    }
  }
}
```

**For development**, create `.env` file:
```dotenv
RAINDROP_ACCESS_TOKEN=your_raindrop_access_token_here
RUST_LOG=info
```

## Usage
Once configured, the MCP server provides the following tools to AI assistants:


### **Collection Management**

- get_collections - List all collections
- get_collection - Get a specific collection
- create_collection - Create a new collection
- update_collection - Update collection properties
- delete_collection - Delete a collection
- share_collection - Share a collection with others

### **Bookmark Operations**

- get_bookmarks - Retrieve bookmarks with filtering
- get_bookmark - Get a specific bookmark
- create_bookmark - Create a new bookmark
- update_bookmark - Update bookmark properties
- delete_bookmark - Delete a bookmark
- search_bookmarks - Search bookmarks with advanced filters
- batch_update_bookmarks - Update multiple bookmarks at once
- batch_delete_bookmarks - Delete multiple bookmarks

### **Tag Management**

- get_tags - List all tags or tags from a specific collection
- rename_tag - Rename a tag across all bookmarks
- delete_tag - Delete a single tag
- delete_tags - Delete multiple tags

### **Highlights**

- get_highlights - Get highlights for a specific bookmark
- get_all_highlights - Get all highlights across bookmarks

### **User & Utility**

- get_user_info - Get user account information
- get_user_stats - Get usage statistics
- empty_trash - Permanently delete all trashed bookmarks
- export_bookmarks - Export bookmarks in various formats
- get_import_status - Check import operation status
- get_export_status - Check export operation status

### ***Example Interactions***
Here are some example prompts you can use with an AI assistant:

- "Show me all my Raindrop collections"
- "Create a new collection called 'AI Research'"
- "Save this article to my 'Reading List' collection"
- "Find all bookmarks tagged with 'rust' and 'programming'"
- "Move all bookmarks from 'Unsorted' to 'Archive' collection"
- "Export my 'Work' collection as a CSV file"
- "Show me all highlights from the past week"

## Requirements

### For End Users (Pre-built Binaries)
- **Operating System**: Linux (x86_64) or Windows (x86_64)
- **Raindrop.io Account**: Valid API access token required

### For Developers (Building from Source)
- **Rust**: 1.85 or later
- **Operating System**: Linux (x86_64) or Windows (x86_64)
- **Raindrop.io Account**: Valid API access token required

## Troubleshooting

### Common Issues

**"Permission denied" when running the binary**
```bash
chmod +x raindrop-mcp-server-linux
```

**"RAINDROP_ACCESS_TOKEN not found"**
- Ensure the token is set in your MCP client configuration
- Verify the token is valid at [Raindrop.io Settings](https://raindrop.io/settings/integrations)

**"Connection refused" or timeout errors**
- Check your internet connection
- Verify Raindrop.io API is accessible from your network

**Claude Desktop not recognizing the server**
- Restart Claude Desktop after configuration changes
- Check `claude_desktop_config.json` syntax with a JSON validator
- Verify the binary path is correct and executable

### Debug Mode

Enable detailed logging:
```bash
RUST_LOG=debug /usr/local/bin/raindrop-mcp-server
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
