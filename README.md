# raindrop-mcp-server-rs
A Model Context Protocol (MCP) server implementation for [Raindrop.io](https://raindrop.io), exposing its API as a set of tools for AI agents and automated workflows.


![Rust 2024](https://img.shields.io/badge/Rust-2024-orange)
[![CI](https://github.com/yamahigashi/raindrop-mcp-server-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/mcp-raindrop/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


---

## Table of Contents
1. [Features](#features)
2. [Quick Start](#quick-start)
3. [Installation](#installation)
   - [From Source](#from-source)
   - [Configuration](#configuration)
4. [Usage](#usage)


---

## Features

| Category  | Highlights |
|-----------|------------|
| **MCP tools** | 40 🛠️ tools auto‑generated via `rmcp‑macros`<br>covering collections, bookmarks, tags, highlights & user endpoints |
| **MCP resources** | Pre‑mounted URIs (`raindrop://collections/all`, `raindrop://tags/all`, …) |
| **MCP prompts** | Ready‑to‑use prompt templates (weekly digest, duplicate finder, etc.) |

## Quick Start
TODO: write later

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yamahigashi/raindrop-mcp-server-rs.git
cd raindrop-mcp-server-rs

# Build the project
cargo build --release

# The binary will be available at ./target/release/raindrop-mcp-server-rs
```

### Configuration
1. Obtain Raindrop.io API Token

Log in to your Raindrop.io account
Go to Settings → Integrations
Create a new app or select an existing one
Generate an access token

2. Set Environment Variables
Create a .env file in the project root:
```dotenv
# Required
RAINDROP_ACCESS_TOKEN=your_raindrop_access_token_here

# Optional
RUST_LOG=info
```

3. Configure MCP Client
Add the server to your MCP client configuration. For Claude Desktop, edit your claude_desktop_config.json:
```json
{
  "mcpServers": {
    "raindrop": {
      "command": "/path/to/mcp-raindrop",
      "env": {
        "RAINDROP_ACCESS_TOKEN": "your_raindrop_access_token_here"
      }
    }
  }
}
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
