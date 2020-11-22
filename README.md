### Installation Guide
#### Ubuntu 20.04
WIP: come back later
##### Compiling CSS
Make sure you have npm install and run the following command from the root
directory of this project, the same directory this README is in.
```bash
npx tailwindcss-cli@latest build resources/postcss/styles.pcss -c resources/postcss/tailwind.config.js -o static/css/styles.css
```
You optionally also install tailwindcss-cli, so you don't need to download it every time you want to recompile the css.
```bash
npm i -g tailwindcss-cli@latest
```
You'll need to make sure npm's global installation directory is in your PATH.
I recommend following this guide: https://github.com/sindresorhus/guides/blob/master/npm-global-without-sudo.md
Once that's all sorted out you can simple run the following command from this directory:
```bash
tailwindcss-cli build resources/postcss/styles.pcss -c resources/postcss/tailwind.config.js -o static/css/styles.css
```

##### Minifying CSS
From in the current directory run the following command:
```bash
npx minify static/css/styles.css > static/css/styles.min.css
```
