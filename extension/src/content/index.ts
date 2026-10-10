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

    let isAvailable = adapter.isEditorAvailable();
    let source = adapter.getSource();
    let language = adapter.getLanguage();

    // Generic fallback for any Monaco or Ace editor if specific adapter did not extract
    if (!source || source.trim().length === 0) {
      const monacoLines = document.querySelectorAll('.view-lines .view-line, .monaco-editor .view-line');
      if (monacoLines.length > 0) {
        source = Array.from(monacoLines).map(l => l.textContent || '').join('\n');
        isAvailable = true;
      }
    }

    if (!source || source.trim().length === 0) {
      const textarea = document.querySelector('textarea.monaco-mouse-cursor-text, textarea#sourceCodeTextarea, textarea[name="code"]') as HTMLTextAreaElement | null;
      if (textarea && textarea.value) {
        source = textarea.value;
        isAvailable = true;
      }
    }

    if (!isAvailable || !source || source.trim().length === 0) {
      return Promise.resolve({
        supported: true,
        editorAvailable: false,
        platformName: adapter.name,
      });
    }

    if (!language) {
      language = (source.includes('public class') || source.includes('System.out') || source.includes('import java')) ? 'java' : 'cpp';
    }

    const constraints = adapter.getConstraints ? adapter.getConstraints() : null;

    return Promise.resolve({
      supported: true,
      editorAvailable: true,
      platformName: adapter.name,
      source: source.trim(),
      language,
      constraints,
    });
  }
});
