import load, * as App from "/client/pkg/app.js";

load().then(() => {
  const app = App.init();
  window.app = app;
});
