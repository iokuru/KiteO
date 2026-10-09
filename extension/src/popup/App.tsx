import { useEffect, useState } from 'preact/hooks';
import browser from 'webextension-polyfill';
import type { AnalysisResult } from '../shared/types';

interface PageState {
  loading: boolean;
  supported: boolean;
  platformName?: string;
  contestMode?: boolean;
  editorAvailable?: boolean;
  error?: string;
}

export function App() {
  const [pageState, setPageState] = useState<PageState>({
    loading: true,
    supported: false,
  });

  const [result, setResult] = useState<AnalysisResult | null>(null);

  useEffect(() => {
    async function loadActivePage() {
      try {
        const tabs = await browser.tabs.query({ active: true, currentWindow: true });
        const activeTab = tabs[0];
        if (!activeTab || !activeTab.id) {
          setPageState({ loading: false, supported: false, error: 'No active tab found' });
          return;
        }

        const response = await browser.tabs.sendMessage(activeTab.id, { type: 'EXTRACT_SOURCE' });

        if (!response || !response.supported) {
          setPageState({
            loading: false,
            supported: false,
            error: 'Navigate to a supported platform editor (Codeforces, LeetCode, CodeChef, AtCoder, etc.)',
          });
          return;
        }

        if (response.contestMode) {
          setPageState({
            loading: false,
            supported: true,
            platformName: response.platformName,
            contestMode: true,
          });
          return;
        }

        if (!response.editorAvailable || !response.source) {
          setPageState({
            loading: false,
            supported: true,
            platformName: response.platformName,
            editorAvailable: false,
            error: 'No code detected in editor.',
          });
          return;
        }

        setPageState({
          loading: false,
          supported: true,
          platformName: response.platformName,
          editorAvailable: true,
        });

        // Request analysis from background worker
        const analysis: AnalysisResult = await browser.runtime.sendMessage({
          type: 'ANALYZE_REQUEST',
          payload: {
            source: response.source,
            language: response.language || 'cpp',
          },
        });

        setResult(analysis);
      } catch (err) {
        setPageState({
          loading: false,
          supported: false,
          error: 'Ready on supported platform tabs (Codeforces, LeetCode, etc.).',
        });
      }
    }

    loadActivePage();
  }, []);

  return (
    <div style={{ padding: '16px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', borderBottom: '1px solid #334155', paddingBottom: '8px' }}>
        <span style={{ fontWeight: 700, fontSize: '15px', letterSpacing: '0.05em' }}>Kite0</span>
        <span style={{ fontSize: '11px', color: '#94a3b8' }}>{pageState.platformName || 'Local CP Analyzer'}</span>
      </div>

      {pageState.loading && (
        <div style={{ padding: '24px 0', textAlign: 'center', color: '#94a3b8', fontSize: '13px' }}>
          Analyzing local editor buffer...
        </div>
      )}

      {!pageState.loading && pageState.contestMode && (
        <div style={{ background: '#451a03', border: '1px solid #b45309', borderRadius: '6px', padding: '12px', color: '#fef3c7', fontSize: '12px' }}>
          <strong>Contest Mode Active</strong>
          <p style={{ margin: '4px 0 0 0' }}>
            Live contest detected. Analyzer is disabled in accordance with platform fair-play rules.
          </p>
        </div>
      )}

      {!pageState.loading && !pageState.contestMode && pageState.error && (
        <div style={{ background: '#1e293b', borderRadius: '6px', padding: '12px', color: '#94a3b8', fontSize: '12px' }}>
          {pageState.error}
        </div>
      )}

      {!pageState.loading && !pageState.contestMode && result && (
        <>
          <div style={{ background: '#1e293b', borderRadius: '6px', padding: '10px' }}>
            <div style={{ fontSize: '11px', color: '#94a3b8', textTransform: 'uppercase', marginBottom: '2px' }}>TC</div>
            <div style={{ fontSize: '16px', fontWeight: 600, color: '#38bdf8' }}>{result.tc}</div>
          </div>

          <div style={{ background: '#1e293b', borderRadius: '6px', padding: '10px' }}>
            <div style={{ fontSize: '11px', color: '#94a3b8', textTransform: 'uppercase', marginBottom: '2px' }}>SC</div>
            <div style={{ fontSize: '16px', fontWeight: 600, color: '#4ade80' }}>{result.sc}</div>
          </div>

          <div style={{ background: '#1e293b', borderRadius: '6px', padding: '10px' }}>
            <div style={{ fontSize: '11px', color: '#94a3b8', textTransform: 'uppercase', marginBottom: '2px' }}>ALGORITHM</div>
            <div style={{ fontSize: '14px', fontWeight: 500, color: '#facc15' }}>
              {result.algorithms.length > 0 ? result.algorithms.join(', ') : 'Unknown'}
            </div>
          </div>

          {result.constraintStatus && (
            <div style={{ background: '#1e293b', borderRadius: '6px', padding: '8px 10px', fontSize: '12px', color: '#c084fc' }}>
              Limits: {result.constraintStatus}
            </div>
          )}
        </>
      )}
    </div>
  );
}
