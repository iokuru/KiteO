import type { PlatformAdapter } from './adapter';
import type { SupportedLanguage, ConstraintStatus } from '../shared/types';

export class LeetCodeAdapter implements PlatformAdapter {
  id = 'leetcode';
  name = 'LeetCode';

  isMatch(url: string): boolean {
    return url.includes('leetcode.com/problems/') || url.includes('leetcode.com/contest/');
  }

  isLiveContest(): boolean {
    return window.location.href.includes('/contest/') && !window.location.href.includes('/ranking');
  }

  isEditorAvailable(): boolean {
    return (
      document.querySelector('.monaco-editor') !== null ||
      document.querySelector('[data-track-load="code_editor"]') !== null ||
      document.querySelector('.view-lines') !== null
    );
  }

  getSource(): string | null {
    // 1. Primary: Extract lines from Monaco editor DOM
    const monacoLines = document.querySelectorAll('.monaco-editor .view-line, .view-lines .view-line');
    if (monacoLines.length > 0) {
      const lines: string[] = [];
      monacoLines.forEach((line) => {
        lines.push(line.textContent || '');
      });
      const combined = lines.join('\n').trim();
      if (combined.length > 0) return combined;
    }

    // 2. Secondary: Monaco hidden/accessible textarea
    const textarea = document.querySelector('textarea.monaco-mouse-cursor-text, .monaco-editor textarea') as HTMLTextAreaElement | null;
    if (textarea && textarea.value && textarea.value.trim().length > 0) {
      return textarea.value;
    }

    // 3. Fallback: check all pre/code blocks in editor container
    const codeBlocks = document.querySelectorAll('.monaco-editor pre, [data-track-load="code_editor"] pre');
    if (codeBlocks.length > 0) {
      const texts = Array.from(codeBlocks).map(b => b.textContent || '').join('\n').trim();
      if (texts.length > 0) return texts;
    }

    return null;
  }

  getLanguage(): SupportedLanguage | null {
    // Check various language dropdown buttons in modern LeetCode UI
    const candidates = [
      document.querySelector('[data-mode-id]'),
      document.querySelector('button[id*="headlessui-listbox-button"]'),
      document.querySelector('[id*="headlessui-popover-button"]'),
      document.querySelector('button[class*="rounded"][class*="text-xs"]'),
      document.querySelector('[data-cy="lang-select"]'),
    ];

    for (const btn of candidates) {
      if (btn && btn.textContent) {
        const text = btn.textContent.toLowerCase();
        if (text.includes('c++') || text.includes('cpp')) return 'cpp';
        if (text.includes('java') && !text.includes('javascript')) return 'java';
        if (text.includes('python')) return 'cpp'; // default parser
      }
    }

    // Infer from source text
    const src = this.getSource() || '';
    if (src.includes('public class') || src.includes('public int') || src.includes('public void') || src.includes('int[]') || src.includes('System.out')) {
      return 'java';
    }
    if (src.includes('vector<') || src.includes('std::') || src.includes('string&') || src.includes('#include') || src.includes('cout <<')) {
      return 'cpp';
    }

    return 'cpp';
  }

  getConstraints(): string | null {
    const el = document.querySelector('[data-track-load="description_content"]');
    if (!el) return null;

    const text = el.textContent || '';
    const idx = text.indexOf('Constraints:');
    if (idx !== -1) {
      return text.substring(idx);
    }
    return null;
  }

  evaluateConstraints(tc: string): ConstraintStatus {
    if (tc === 'Unknown') return "Can't tell";

    const constraints = this.getConstraints();
    if (!constraints) return "Can't tell";

    // Extract maximum N bound (e.g. 10^5, 2 * 10^5, 1000, 200, 20)
    let maxN = 100000;
    const match105 = constraints.match(/10\^5|100000/);
    const match2x105 = constraints.match(/2\s*\*\s*10\^5|200000/);
    const match104 = constraints.match(/10\^4|10000/);
    const match1000 = constraints.match(/1000|10\^3/);
    const match200 = constraints.match(/200|300|500/);
    const match20 = constraints.match(/15|16|18|20|22/);

    if (match2x105) maxN = 200000;
    else if (match105) maxN = 100000;
    else if (match104) maxN = 10000;
    else if (match1000) maxN = 1000;
    else if (match200) maxN = 300;
    else if (match20) maxN = 20;

    let ops = 0;
    if (tc == 'O(n)') {
      ops = maxN;
    } else if (tc == 'O(n log n)') {
      ops = maxN * Math.log2(maxN);
    } else if (tc == 'O(n^2)') {
      ops = maxN * maxN;
    } else if (tc.includes('2^n')) {
      ops = Math.pow(2, Math.min(maxN, 30)) * maxN;
    } else {
      return "Can't tell";
    }

    if (ops <= 100000000) {
      return 'Within limits';
    } else if (ops > 1000000000) {
      return 'Likely too slow';
    }
    return "Can't tell";
  }
}
