import type { PlatformAdapter } from './adapter';
import type { SupportedLanguage } from '../shared/types';

export class AtCoderAdapter implements PlatformAdapter {
  id = 'atcoder';
  name = 'AtCoder';

  isMatch(url: string): boolean {
    return url.includes('atcoder.jp');
  }

  isEditorAvailable(): boolean {
    return (
      document.querySelector('.plain-textarea') !== null ||
      document.querySelector('textarea[name*="sourceCode"]') !== null ||
      document.querySelector('.CodeMirror') !== null
    );
  }

  getSource(): string | null {
    const cm = document.querySelector('.CodeMirror') as any;
    if (cm && cm.CodeMirror) {
      return cm.CodeMirror.getValue();
    }
    const textarea = document.querySelector('textarea[name*="sourceCode"], .plain-textarea') as HTMLTextAreaElement | null;
    return textarea ? textarea.value : null;
  }

  getLanguage(): SupportedLanguage | null {
    const select = document.querySelector('select[name*="language"]') as HTMLSelectElement | null;
    if (select) {
      const text = select.options[select.selectedIndex]?.text?.toLowerCase() || '';
      if (text.includes('java')) return 'java';
    }
    return 'cpp';
  }
}

export class HackerRankAdapter implements PlatformAdapter {
  id = 'hackerrank';
  name = 'HackerRank';

  isMatch(url: string): boolean {
    return url.includes('hackerrank.com');
  }

  isEditorAvailable(): boolean {
    return document.querySelector('.monaco-editor') !== null;
  }

  getSource(): string | null {
    const lines = document.querySelectorAll('.monaco-editor .view-line');
    if (lines.length > 0) {
      return Array.from(lines).map((l) => l.textContent || '').join('\n');
    }
    return null;
  }

  getLanguage(): SupportedLanguage | null {
    const select = document.querySelector('[data-automation="code-editor-language-select"]');
    const text = select?.textContent?.toLowerCase() || '';
    if (text.includes('java')) return 'java';
    return 'cpp';
  }
}

export class GeeksforGeeksAdapter implements PlatformAdapter {
  id = 'geeksforgeeks';
  name = 'GeeksforGeeks';

  isMatch(url: string): boolean {
    return url.includes('geeksforgeeks.org');
  }

  isEditorAvailable(): boolean {
    return document.querySelector('.monaco-editor') !== null || document.querySelector('.ace_editor') !== null;
  }

  getSource(): string | null {
    const lines = document.querySelectorAll('.view-line, .ace_line');
    if (lines.length > 0) {
      return Array.from(lines).map((l) => l.textContent || '').join('\n');
    }
    return null;
  }

  getLanguage(): SupportedLanguage | null {
    const langBtn = document.querySelector('[class*="language-select"]');
    const text = langBtn?.textContent?.toLowerCase() || '';
    if (text.includes('java')) return 'java';
    return 'cpp';
  }
}

export class HackerEarthAdapter implements PlatformAdapter {
  id = 'hackerearth';
  name = 'HackerEarth';

  isMatch(url: string): boolean {
    return url.includes('hackerearth.com');
  }

  isEditorAvailable(): boolean {
    return document.querySelector('.ace_editor') !== null || document.querySelector('.monaco-editor') !== null;
  }

  getSource(): string | null {
    const lines = document.querySelectorAll('.ace_line, .view-line');
    if (lines.length > 0) {
      return Array.from(lines).map((l) => l.textContent || '').join('\n');
    }
    return null;
  }

  getLanguage(): SupportedLanguage | null {
    const select = document.querySelector('#select-language') as HTMLSelectElement | null;
    const text = select?.options[select.selectedIndex]?.text?.toLowerCase() || '';
    if (text.includes('java')) return 'java';
    return 'cpp';
  }
}

export class CsesAdapter implements PlatformAdapter {
  id = 'cses';
  name = 'CSES';

  isMatch(url: string): boolean {
    return url.includes('cses.fi');
  }

  isEditorAvailable(): boolean {
    return document.querySelector('textarea[name="code"]') !== null;
  }

  getSource(): string | null {
    const textarea = document.querySelector('textarea[name="code"]') as HTMLTextAreaElement | null;
    return textarea ? textarea.value : null;
  }

  getLanguage(): SupportedLanguage | null {
    const select = document.querySelector('select[name="lang"]') as HTMLSelectElement | null;
    const text = select?.options[select.selectedIndex]?.text?.toLowerCase() || '';
    if (text.includes('java')) return 'java';
    return 'cpp';
  }
}
