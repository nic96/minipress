### Installation Guide
#### Ubuntu 20.04
WIP: come back later
##### Compiling CSS
Make sure you have yarn installed and run the following commands from the root
directory of this project, the same directory this README is in.
```bash
yarn global add tailwindcss-cli@latest --prefix /usr/local
yarn install
```

Then run the following command to compile the css.
```bash
tailwindcss-cli build resources/postcss/styles.pcss -c resources/postcss/tailwind.config.js -o static/css/styles.css
```

##### Minifying CSS
There are different ways of minify the css. Here's one option:
```bash
yarn global add minify --prefix /usr/local
minify static/css/styles.css > static/css/styles.min.css
```
