# AWS Documents Language Server

Language server for AWS File types

## Functionality

This Language Server currently supports completion, hover, diagnostics for the following file types:
- Buildspec

## Structure

```
.
── src
    └── server.ts // Language Server entry point
    └── registry.ts // Registry of items
    └── service.ts // Interfaces for Language Service and implementation of Backend Services that we call into (json language service, yaml language service)
    └── utils/
        └── file.ts // Utilities for interacting with files
    └── filetypes/ // Folder where filetype implementations live 
```