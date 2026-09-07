---
name: tunnelto
description: >-
  Expose a locally running web server to the internet with a public HTTPS URL
  using the tunnelto CLI (tunnelto.dev). Use when you need to share a dev
  server, receive webhooks (Stripe, Slack, GitHub, Twilio) on localhost, test a
  mobile app against a local backend, or give another agent or person a public
  URL for something running on this machine. Triggers: "tunnel", "expose
  localhost", "public URL", "ngrok alternative", "webhook to localhost",
  "tunnelto", "tunn.dev".
metadata:
  source: https://github.com/tunneltodev/tunnelto/tree/main/skills/tunnelto
  homepage: https://tunnelto.dev
  agents_md: https://tunnelto.dev/AGENTS.md
---

# tunnelto

`tunnelto` is a single-binary CLI that opens a tunnel from a public
`https://<subdomain>.tunnelto.dev` URL to a port on this machine. It runs in the
foreground and keeps the tunnel open until the process exits.

## 1. Check for and install the CLI

```bash
command -v tunnelto || curl -sL https://tunnelto.dev/install.sh | sh
```

The installer puts the binary in `~/.tunnelto/bin`; if `tunnelto` is still not
on `PATH` afterwards, use `~/.tunnelto/bin/tunnelto` or export
`PATH="$HOME/.tunnelto/bin:$PATH"`. Alternatives: `brew install
tunneltodev/tap/tunnelto`, `cargo install tunnelto`, or a release binary from
https://github.com/tunneltodev/tunnelto/releases.

## 2. Authenticate (optional but recommended)

An access key comes from https://tunnelto.dev/dashboard. Store it once:

```bash
tunnelto set-auth --key <ACCESS_KEY>
```

The key is saved to `~/.tunnelto/key.token` and used automatically. To use a
key for a single run instead, pass `--key <ACCESS_KEY>`. Without a key the CLI
still works: it opens an anonymous tunnel on a generated subdomain, and
`--subdomain` is ignored.

## 3. Open a tunnel

```bash
# forward https://<random>.tunnelto.dev -> http://localhost:8000
tunnelto --port 8000

# with a reserved subdomain (paid plan): https://myapp.tunnelto.dev
tunnelto --port 8000 --subdomain myapp
```

The process blocks. From an agent, run it in the background, write its output
to a file, and read the public URL back from that file:

```bash
NO_COLOR=1 tunnelto --port 8000 > /tmp/tunnelto.log 2>&1 &
echo $! > /tmp/tunnelto.pid

# wait for the URL (all status output is on stderr; it is merged above)
for i in $(seq 1 30); do
  URL=$(grep -oE 'https://[a-z0-9-]+\.tunnelto\.dev' /tmp/tunnelto.log | head -1)
  [ -n "$URL" ] && break
  sleep 1
done
echo "$URL"
```

On success the log contains a small table with `Public tunnel URL`,
`Local inspect dashboard` (a localhost port that shows every request through the
tunnel), and `Forwarding traffic to`. Anything served at the public URL is
proxied to the local port, including WebSockets.

Close the tunnel by killing the process:

```bash
kill "$(cat /tmp/tunnelto.pid)"
```

## Flags

| Flag | Meaning | Default |
| --- | --- | --- |
| `-p, --port <PORT>` | Local port to forward to | `8000` |
| `-s, --subdomain <NAME>` | Requested subdomain (needs a key and a reserved subdomain) | generated |
| `-k, --key <KEY>` | Access key for this run (overrides the stored key) | stored key |
| `--host <HOST>` | Local host to forward to | `localhost` |
| `-t, --use-tls` | Forward to `https://<host>:<port>` instead of `http://` | off |
| `--dashboard-port <PORT>` | Fixed port for the local inspect dashboard | random |
| `-v, --verbose` | Debug logging | off |
| `set-auth --key <KEY>` | Subcommand: store the key on disk and exit | |

## Recipes

- **Receive webhooks on localhost.** Start the local server, open a tunnel, then
  register `$URL/<webhook-path>` with the provider (Stripe, Slack, GitHub,
  Twilio). Watch the local inspect dashboard URL from the log to see each
  incoming request and response.
- **Share a dev server with a person.** Open a tunnel and send them `$URL`.
  The URL is live only while the process runs.
- **Mobile app against a local backend.** Point the app's API base URL at `$URL`.
- **Local HTTPS server.** Add `--use-tls` so the tunnel forwards to
  `https://localhost:<port>`.

## Gotchas

- Everything the CLI prints goes to **stderr**, not stdout. Merge with
  `2>&1` or read stderr when capturing the URL. Set `NO_COLOR=1` to strip
  ANSI colour codes.
- The tunnel URL only exists while the process is alive; do not exit the shell
  that started it unless it was backgrounded with `nohup`/`&`.
- Anonymous tunnels get a generated subdomain. Custom subdomains require an
  access key and a subdomain reserved at https://tunnelto.dev/dashboard.
  A mismatch prints a `>>> Notice` line but the tunnel still opens on the
  generated name, so always read the URL from the log rather than assuming it.
- Paid plans allow up to 20 reserved subdomains and 5 concurrent tunnels per
  user. Exceeding limits prints `Server terminated connection: ...` and the
  process exits with status 1.
- `Error: AuthenticationFailed` means the stored or passed key is wrong; get a
  fresh one from the dashboard and re-run `tunnelto set-auth --key`.
- Nothing is cached locally besides `~/.tunnelto/key.token`; deleting that
  directory fully resets the CLI.

## Self-hosting

The server is open source (https://github.com/agrinman/tunnelto). Point the
client at your own control server with environment variables:
`CTRL_HOST` (default `wormhole.tunnelto.dev`), `CTRL_PORT` (default `443`), and
`CTRL_TLS_OFF=1` to use plain `ws://` and `http://`.

## Installing this skill

```bash
npx skills add tunneltodev/tunnelto            # skills CLI (Claude Code, Codex, Cursor, ...)
# or fetch the file directly
mkdir -p ~/.claude/skills/tunnelto && curl -sL https://tunnelto.dev/skills/tunnelto/SKILL.md -o ~/.claude/skills/tunnelto/SKILL.md
```

## Links

- Site and dashboard: https://tunnelto.dev, https://tunnelto.dev/dashboard
- Agent guide: https://tunnelto.dev/AGENTS.md
- Source: https://github.com/tunneltodev/tunnelto
- Support: support@tunnelto.dev
