import type { PlatformAdapter } from './adapter';
import type { SupportedLanguage } from '../shared/types';

export class OnlineGdbAdapter implements PlatformAdapter {
  id = 'onlinegdb';
  name = 'OnlineGDB';

  isMatch(url: string): boolean {
    return url.includes('onlinegdb.com');
  }

  isEditorAvailable(): boolean {
    return document.querySelector('#editor') !== null || document.querySelector('.ace_editor') !== null;
  }

  getSource(): string | null {
    const lines = document.querySelectorAll('.ace_line');
    if (lines.length > 0) {
      return Array.from(lines).map((l) => l.textContent || '').join('\n');
    }
    return null;
  }

  getLanguage(): SupportedLanguage | null {
    const select = document.querySelector('#select_lang') as HTMLSelectElement | null;
    const text = select?.options[select.selectedIndex]?.text?.toLowerCase() || '';
    if (text.includes('java')) return 'java';
    return 'cpp';
  }
}

export class JDoodleAdapter implements PlatformAdapter {
  id = 'jdoodle';
  name = 'JDoodle';

  isMatch(url: string): boolean {
    return url.includes('jdoodle.com');
  }

  isEditorAvailable(): boolean {
    return document.querySelector('.ace_editor') !== null;
  }

  getSource(): string | null {
    const lines = document.querySelectorAll('.ace_line');
    if (lines.length > 0) {
      return Array.from(lines).map((l) => l.textContent || '').join('\n');
    }
    return null;
  }

  getLanguage(): SupportedLanguage | null {
    return window.location.href.includes('java') ? 'java' : 'cpp';
  }
}

export class ProgramizAdapter implements PlatformAdapter {
  id = 'programiz';
  name = 'Programiz';

  isMatch(url: string): boolean {
    return url.includes('programiz.com');
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
    return window.location.href.includes('java') ? 'java' : 'cpp';
  }
}

export class ReplitAdapter implements PlatformAdapter {
  id = 'replit';
  name = 'Replit';

  isMatch(url: string): boolean {
    return url.includes('replit.com');
  }

  isEditorAvailable(): boolean {
    return document.querySelector('.monaco-editor') !== null || document.querySelector('.cm-content') !== null;
  }

  getSource(): string | null {
    const lines = document.querySelectorAll('.view-line, .cm-line');
    if (lines.length > 0) {
      return Array.from(lines).map((l) => l.textContent || '').join('\n');
    }
    return null;
  }

  getLanguage(): SupportedLanguage | null {
    const src = this.getSource() || '';
    if (src.includes('public class') || src.includes('System.out.println')) return 'java';
    return 'cpp';
  }
}
