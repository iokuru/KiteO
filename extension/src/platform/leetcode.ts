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
    // 1. Primary: Locate the specific Solution code editor container
    // Avoid console, testcase, or test-result sub-editors
    const codeEditorContainers = [
      document.querySelector('[data-track-load="code_editor"]'),
      document.querySelector('[data-mode-id="cpp"], [data-mode-id="java"], [data-mode-id="python3"], [data-mode-id="c"]')?.closest('.monaco-editor'),
      document.querySelector('.editor-container .monaco-editor'),
    ];

    for (const container of codeEditorContainers) {
      if (container) {
        const lines = container.querySelectorAll('.view-lines .view-line, .view-line');
        if (lines.length > 0) {
          const text = Array.from(lines)
            .map((l) => (l.textContent || '').replace(/\u00a0/g, ' '))
            .join('\n')
            .trim();
          if (text.length > 0) return text;
        }
      }
    }

    // 2. Iterate all monaco-editor instances and select the one with actual solution code
    const allEditors = document.querySelectorAll('.monaco-editor');
    let bestCandidate: string | null = null;
    let maxLines = 0;

    for (const ed of Array.from(allEditors)) {
      if (
        ed.closest('#console') ||
        ed.closest('[class*="console"]') ||
        ed.closest('[class*="testcase"]') ||
        ed.closest('[class*="test-result"]')
      ) {
        continue;
      }

      const lines = ed.querySelectorAll('.view-lines .view-line, .view-line');
      if (lines.length > 0) {
        const text = Array.from(lines)
          .map((l) => (l.textContent || '').replace(/\u00a0/g, ' '))
          .join('\n')
          .trim();
        if (text.includes('Solution') || text.includes('int ') || text.includes('void ') || text.includes('for')) {
          return text;
        }
        if (lines.length > maxLines) {
          maxLines = lines.length;
          bestCandidate = text;
        }
      }
    }

    if (bestCandidate && bestCandidate.length > 0) {
      return bestCandidate;
    }

    // 3. Fallback to generic line extraction
    const monacoLines = document.querySelectorAll(
      '.monaco-editor .view-line, .view-lines .view-line, [role="code"] .view-line, .lines-content .view-line'
    );
    if (monacoLines.length > 0) {
      const lines: string[] = [];
      monacoLines.forEach((line) => {
        lines.push((line.textContent || '').replace(/\u00a0/g, ' '));
      });
      const combined = lines.join('\n').trim();
      if (combined.length > 0) return combined;
    }

    return null;
  }

  getLanguage(): SupportedLanguage | null {
    const src = this.getSource() || '';
    // Priority 1: Direct language markers in source
    if (
      src.includes('vector<') ||
      src.includes('public:') ||
      src.includes('#include') ||
      src.includes('std::') ||
      src.includes('cout <<') ||
      src.includes('int&')
    ) {
      return 'cpp';
    }
    if (
      src.includes('public class') ||
      src.includes('System.out') ||
      src.includes('import java') ||
      src.includes('String[]')
    ) {
      return 'java';
    }

    // Priority 2: Language dropdown in modern LeetCode UI
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
      }
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
