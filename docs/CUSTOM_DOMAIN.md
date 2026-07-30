# Connect cactus-browser.tech to the landing page

Hosting uses **GitHub Pages** from this repo (`main` → GitHub Actions workflow `Deploy landing page`).

## 1. GitHub (one time)

1. Open [github.com/yohannes-mesay/cactus-browser/settings/pages](https://github.com/yohannes-mesay/cactus-browser/settings/pages)
2. **Build and deployment** → Source: **GitHub Actions**
3. After the first successful workflow run, under **Custom domain** enter: `cactus-browser.tech`
4. Enable **Enforce HTTPS** when GitHub offers it (after DNS propagates — usually a few minutes after “DNS check successful”)

GitHub may still show the default `yohannes-mesay.github.io/cactus-browser/` URL in Actions; once the custom domain is saved, the live site is **https://cactus-browser.tech**.

## 2. DNS at manage.get.tech

Keep your **default Orderbox nameservers** (no need to change the Nameservers tab). Open **DNS** → **DNS Records** and add:

### Apex (cactus-browser.tech)

| Type | Host / Name | Value |
|------|-------------|--------|
| A | `@` (or blank) | `185.199.108.153` |
| A | `@` | `185.199.109.153` |
| A | `@` | `185.199.110.153` |
| A | `@` | `185.199.111.153` |

### www (optional)

| Type | Host / Name | Value |
|------|-------------|--------|
| CNAME | `www` | `yohannes-mesay.github.io` |

Some panels want a trailing dot on the CNAME target: `yohannes-mesay.github.io.`

## 3. Verify

- Wait 5–60 minutes for DNS (sometimes up to 24h).
- GitHub → repo **Settings → Pages** should show “DNS check successful”.
- Visit [https://cactus-browser.tech](https://cactus-browser.tech)

## 4. Redeploy after editing the landing page

Edit `resources/landing.html`, push to `main`. The workflow bakes placeholders and publishes automatically.

Local preview:

```bash
bash scripts/bake-site.sh
python3 -m http.server 8080 --directory site
# open http://127.0.0.1:8080/
```
