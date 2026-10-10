import browser from 'webextension-polyfill';
import { getAdapterForUrl } from '../platform/registry';

function extractCodeFromPage() {
  const adapter = getAdapterForUrl(window.location.href);
  if (!adapter) {
    return {
      supported: false,
      error: 'Unsupported platform',
    };
  }

  if (adapter.isLiveContest && adapter.isLiveContest()) {
    return {
      supported: true,
      contestMode: true,
      platformName: adapter.name,
    };
  }

  let isAvailable = adapter.isEditorAvailable();
  let source = adapter.getSource();
  let language = adapter.getLanguage();

  // Generic fallback for any Monaco or Ace or CodeMirror or LeetCode editor
  if (!source || source.trim().length === 0) {
    const monacoLines = document.querySelectorAll(
      '.view-lines .view-line, .monaco-editor .view-line, [role="code"] .view-line, .lines-content .view-line'
    );
    if (monacoLines.length > 0) {
      source = Array.from(monacoLines)
        .map((l) => (l.textContent || '').replace(/\u00a0/g, ' '))
        .join('\n');
      isAvailable = true;
    }
  }

  if (!source || source.trim().length === 0) {
    const textarea = document.querySelector(
      'textarea.monaco-mouse-cursor-text, textarea.inputarea, textarea#sourceCodeTextarea, textarea[name="code"]'
    ) as HTMLTextAreaElement | null;
    if (textarea && textarea.value) {
      source = textarea.value;
      isAvailable = true;
    }
  }

  if (!isAvailable || !source || source.trim().length === 0) {
    return {
      supported: true,
      editorAvailable: false,
      platformName: adapter.name,
    };
  }

  source = source.replace(/\u00a0/g, ' ').trim();

  // Smart override for language if source contains explicit C++ or Java tokens
  if (
    source.includes('vector<') ||
    source.includes('public:') ||
    source.includes('#include') ||
    source.includes('std::') ||
    source.includes('cout <<') ||
    source.includes('int&')
  ) {
    language = 'cpp';
  } else if (
    source.includes('public class') ||
    source.includes('System.out') ||
    source.includes('import java') ||
    source.includes('String[]')
  ) {
    language = 'java';
  } else if (!language) {
    language = 'cpp';
  }

  const constraints = adapter.getConstraints ? adapter.getConstraints() : null;

  return {
    supported: true,
    editorAvailable: true,
    platformName: adapter.name,
    source,
    language,
    constraints,
  };
}

// 1. webextension-polyfill listener
browser.runtime.onMessage.addListener((message) => {
  if (message.type === 'EXTRACT_SOURCE') {
    return Promise.resolve(extractCodeFromPage());
  }
});

// 2. Native chrome.runtime listener fallback
if (typeof chrome !== 'undefined' && chrome.runtime && chrome.runtime.onMessage) {
  try {
    chrome.runtime.onMessage.addListener((message, _sender, sendResponse) => {
      if (message.type === 'EXTRACT_SOURCE') {
        sendResponse(extractCodeFromPage());
        return true;
      }
    });
  } catch {
    // Ignore if already hooked
  }
}
