import browser from 'webextension-polyfill';

browser.runtime.onMessage.addListener((message) => {
  if (message.type === 'EXTRACT_SOURCE') {
    return Promise.resolve({
      source: null,
      language: null,
      constraints: null,
    });
  }
});
