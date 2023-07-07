# Micro-RDK Webhook Template

## TODO
- update scripts to use LED example
- add wiring instructions
- update scripts for better connection cleanup
- Jaeger instructions

## Deploy Webhook with Fly.io

Follow steps 1, 2, and 3 to use fly.io's [`flyctl`](https://fly.io/docs/hands-on/install-flyctl/) tool.

1. `$ cd <my-webhook-project>`
2. `$ fly launch`; this is only done once, for updating apps use `fly deploy`
3. Note your new app's `url`, ex. `https://fly.io/apps/my-webhook-app` 

## Add Webhook Configs to Robot

The following steps take place in your [`app.viam`](https://app.viam.com) account.

1. Add a new robot, ex. `webhook-robot`
2. In your robot's `Config` tab, select the `Raw JSON` option and paste the json template below
3. Replace `<my-webhook-url>` with your fly.io app's url and append one of the following paths to it
    - `/py` to execute `hook.py`, ex. `https://my-app-url/py`
    - `/go` to execute `gohook/hook.go`, ex. `https://my-app-url/go`
4. Replace `<my-robot-location-secret>` with your location secret found in your robot's `Code Sample` example
5. Save your config

> Note: The template's SDK scripts, (i.e. `hook.py`, `hook.go`) assume your board is named `board`.

```json
{
  "components": [
    {
      "name": "board",
      "type": "board",
      "model": "esp32",
      "attributes": {
        "webhook": <my-webhook-url>,
        "webhook-secret": <my-robot-location-secret>, 
        "pins": [
          15
        ]
      }
    }
  ] 
}
```

## Monitoring 

Webhook Logs: "https://my-fly-io-app/monitoring"