# Julkaisu GitHubiin

```bash
cd ein-sof
# vaihda KAYTTAJA omaksi GitHub-käyttäjänimeksi:
sed -i 's/KAYTTAJA/oma-kayttaja/g' docs/index.md      # macOS: sed -i '' ...
gh auth login
gh repo create ein-sof --public --source=. --push \
  --description "EIN SOF — EMET — GAMMAWHITE.com"
git push --tags
# GitHub Pages docs/-kansiosta:
gh api -X POST repos/{owner}/ein-sof/pages -f "source[branch]=main" -f "source[path]=/docs"
```

DNS (gammawhite.com): A-tietueet 185.199.108.153, .109.153, .110.153, .111.153 ja `www` CNAME → `oma-kayttaja.github.io`.
