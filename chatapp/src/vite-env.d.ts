/// <reference types="vite/client" />
// Add data types to window.navigator for use in this file. See https://www.typescriptlang.org/docs/handbook/triple-slash-directives.html#-reference-types- for more info.
/// <reference types="user-agent-data-types" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}
