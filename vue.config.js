module.exports = {
  css: {
    requireModuleExtension: false,
    loaderOptions: {
      sass: {
        // Import global variables here, import all the other (s)css
        // via src/App.vue -> <style>.
        prependData: `@import "~@/style/bootstrap-pre.scss";`
      }
    }
  }
};
