// The standard set, plus the selector rules of the HTML and CSS rulebook:
// style by class, never by id, never with !important, and stop a selector
// at three levels.
export default {
  extends: ["stylelint-config-standard"],
  rules: {
    "selector-max-id": 0,
    "declaration-no-important": true,
    "selector-no-qualifying-type": true,
    "selector-max-compound-selectors": 3,
    // BlinkMacSystemFont is the conventional spelling of Chrome's system
    // font keyword on macOS.
    "value-keyword-case": ["lower", { ignoreKeywords: ["BlinkMacSystemFont"] }],
  },
};
