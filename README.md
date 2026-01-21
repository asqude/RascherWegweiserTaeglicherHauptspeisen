# RascherWegweiserTaeglicherHauptspeisen

A simple CLI tool that displays today's "Klassiker" from the Mensa Academica Aachen.

## Installation

### NixOS (Flake)

Add the flake as an input in your `flake.nix`:

```nix
{
  inputs = {    
    dishdetector.url = "github:asqude/RascherWegweiserTaeglicherHauptspeisen";
  };
}
```

Then add the package to your system or home-manager packages:

```nix
# In your NixOS configuration:
environment.systemPackages = [
  inputs.dishdetector.packages.${pkgs.stdenv.hostPlatform.system}.default
];

# Or in home-manager:
home.packages = [
  inputs.dishdetector.packages.${pkgs.stdenv.hostPlatform.system}.default
];
```

### Build from source

```bash
git clone https://github.com/asqude/RascherWegweiserTaeglicherHauptspeisen.git
cd RascherWegweiserTaeglicherHauptspeisen
nix build
# or
cargo build --release
```

## Usage

Simply run the command:

```bash
RascherWegweiserTaeglicherHauptspeisen
```

Example output:
```
Hähnchennuggets | Curry-Mango-Dip
```

## Waybar Integration

Add a custom module to your Waybar configuration to display the daily Klassiker.

### Waybar Config (`~/.config/waybar/config`)

Add a custom module:

```json
{
  "modules-right": ["custom/mensa"],
  
  "custom/mensa": {
    "exec": "RascherWegweiserTaeglicherHauptspeisen",
    "interval": 43200,
    "format": "{}",
    "tooltip": true,
    "return-type": "",
    "on-click": "xdg-open https://www.studierendenwerk-aachen.de/de/Gastronomie/mensa-academica-wochenplan.html"
  }
}
```

**Configuration options:**
- `interval`: 43200 seconds = 12 hours
- `on-click`: Opens the full menu in your browser

### NixOS/Home-Manager Waybar Config

If you configure Waybar via Home-Manager:

```nix
programs.waybar = {
  enable = true;
  settings = {
    mainBar = {
      modules-right = [ "custom/mensa" ];
      
      "custom/mensa" = {
        exec = "RascherWegweiserTaeglicherHauptspeisen";
        interval = 43200;  # 12 hours
        format = "{}";
        on-click = "xdg-open https://www.studierendenwerk-aachen.de/de/Gastronomie/mensa-academica-wochenplan.html";
      };
    };
  };
};
```

## License

MIT