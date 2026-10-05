import browser from 'webextension-polyfill';
import { getAdapterForUrl } from '../platform/registry';

browser.runtime.onMessage.addListener((message) => {
  if (message.type === 'EXTRACT_SOURCE') {
    const adapter = getAdapterForUrl(window.location.href);
    if (!adapter) {
      return Promise.resolve({
        supported: false,
        error: 'Unsupported platform',
      });
    }

    if (adapter.isLiveContest && adapter.isLiveContest()) {
      return Promise.resolve({
        supported: true,
        contestMode: true,
        platformName: adapter.name,
      });
    }

    if (!adapter.isEditorAvailable()) {
      return Promise.resolve({
        supported: true,
        editorAvailable: false,
        platformName: adapter.name,
      });
    }

    const source = adapter.getSource();
    const language = adapter.getLanguage();
    const constraints = adapter.getConstraints ? adapter.getConstraints() : null;

    return Promise.resolve({
      supported: true,
      editorAvailable: true,
      platformName: adapter.name,
      source,
      language,
      constraints,
    });
  }
});
