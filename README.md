# passgen

A fast, flexible, secure password generator written in Rust.  
Built for instant use via the command line and Alfred workflows.

## Features

- **Default mode**:  
  Generates a readable, strong passphrase using:
  - 4 mixed English words  
  - Random capitalization  
  - Dashes  
  - 2 digits at the end  
  - Example: `River-Bright-Mosaic-Sunset-84`

- **Random-character mode (full charset)**: Includes letters, digits, and special characters.
- **Random-character mode (letters + digits)**


## Installation

### From source

```bash
git clone <your_repo>
cd passgen
cargo install --path .
```

## Usage

- `passgen default`: `Querier-Posture-Pioneer-trimly-27`
- `passgen chars 32`: `|1ktsI4y2sfF"@nEFnP"Kh8&Z)D5>O$*`
- `passgen chars-ns 24`: `vkICZ6BcMP1PLEm4WZ1YyLpe`
