This Program Requires sudo to work! (Since it can't automatically run openvpn profiles without it.)
Upon first run it will create a config.json file automatically filled with a preset
You'll have you populate the config for your own ovpn files for it to be useful for you.

# Default Config
``` json
{
  "vpn_files": [
    {
      "name": "Example VPN 1",
      "path": "/path/to/example_vpn1.ovpn"
    },
    {
      "name": "Example VPN 2",
      "path": "/path/to/example_vpn2.ovpn"
    }
  ]
}
```