import type { PlatformAdapter } from './adapter';
import type { SupportedLanguage } from '../shared/types';

export class CodeChefAdapter implements PlatformAdapter {
  id = 'codechef';
  name = 'CodeChef';

  isMatch(url: string): boolean {
    return url.includes('codechef.com');
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
    const langEl = document.querySelector('[class*="language"]') || document.querySelector('#select-language');
    const text = langEl?.textContent?.toLowerCase() || '';
    if (text.includes('java')) return 'java';
    return 'cpp';
  }
}
