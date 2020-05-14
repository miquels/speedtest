module.exports = {
  css: {
    requireModuleExtension: false,
    loaderOptions: {
      sass: {
        prependData: `@import "~@/style/style.scss";`
      }
    }
  }
};
