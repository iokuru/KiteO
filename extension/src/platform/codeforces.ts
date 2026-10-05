import type { PlatformAdapter } from './adapter';
import type { SupportedLanguage } from '../shared/types';

export class CodeforcesAdapter implements PlatformAdapter {
  id = 'codeforces';
  name = 'Codeforces';

  isMatch(url: string): boolean {
    return url.includes('codeforces.com/contest/') || url.includes('codeforces.com/problemset/');
  }

  isLiveContest(): boolean {
    const isContestUrl = window.location.href.includes('/contest/');
    const countdown = document.querySelector('#countdown');
    return isContestUrl && countdown !== null;
  }

  isEditorAvailable(): boolean {
    return (
      document.querySelector('textarea#sourceCodeTextarea') !== null ||
      document.querySelector('.ace_editor') !== null
    );
  }

  getSource(): string | null {
    const ace = document.querySelector('.ace_editor') as HTMLElement | null;
    if (ace) {
      const lines = ace.querySelectorAll('.ace_line');
      if (lines.length > 0) {
        return Array.from(lines).map((l) => l.textContent || '').join('\n');
      }
    }

    const textarea = document.querySelector('textarea#sourceCodeTextarea') as HTMLTextAreaElement | null;
    return textarea ? textarea.value : null;
  }

  getLanguage(): SupportedLanguage | null {
    const select = document.querySelector('select[name="programTypeId"]') as HTMLSelectElement | null;
    if (select) {
      const selected = select.options[select.selectedIndex]?.text?.toLowerCase() || '';
      if (selected.includes('c++') || selected.includes('gnu g++')) return 'cpp';
      if (selected.includes('java')) return 'java';
    }

    const src = this.getSource() || '';
    if (src.includes('import java') || src.includes('public class Main')) return 'java';
    return 'cpp';
  }
}
