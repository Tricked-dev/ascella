const tailwindcss = require("tailwindcss");
const autoprefixer = require("autoprefixer");
const csso = require("postcss-csso");
// const cssnano = require('cssnano');
const config = {
  plugins: [
    tailwindcss(),
    autoprefixer(),
    csso(),
    // cssnano()
  ],
};

module.exports = config;
