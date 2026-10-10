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

function Kite0Badge() {
  return (
    <svg width="22" height="22" viewBox="0 0 24 24" fill="none" style={{ flexShrink: 0 }}>
      <circle cx="12" cy="12" r="11" fill="#16a34a" />
      <polygon points="12,4 17.5,11.5 12,19.5 6.5,11.5" stroke="#ffffff" strokeWidth="1.8" fill="none" strokeLinejoin="round" />
      <line x1="12" y1="4" x2="12" y2="19.5" stroke="#ffffff" strokeWidth="1.1" strokeOpacity="0.8" />
      <line x1="6.5" y1="11.5" x2="17.5" y2="11.5" stroke="#ffffff" strokeWidth="1.1" strokeOpacity="0.8" />
    </svg>
  );
}

const MONO_FONT = 'ui-monospace, SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace';

export function App() {
  const [pageState, setPageState] = useState<PageState>({
    loading: true,
    supported: false,
  });

  const [result, setResult] = useState<AnalysisResult | null>(null);
  const [copied, setCopied] = useState(false);

  async function analyzeTab() {
    setPageState(prev => ({ ...prev, loading: true, error: undefined }));
    try {
      const tabs = await browser.tabs.query({ active: true, currentWindow: true });
      const activeTab = tabs[0];
      if (!activeTab || !activeTab.id) {
        setPageState({ loading: false, supported: false, error: 'No active browser tab found.' });
        return;
      }

      let response: any = null;
      try {
        response = await browser.tabs.sendMessage(activeTab.id, { type: 'EXTRACT_SOURCE' });
      } catch {
        // Fallback to injection if content script wasn't pre-loaded
      }

      if (!response && (browser as any).scripting?.executeScript) {
        try {
          await (browser as any).scripting.executeScript({
            target: { tabId: activeTab.id },
            files: ['content.js']
          });
          response = await browser.tabs.sendMessage(activeTab.id, { type: 'EXTRACT_SOURCE' });
        } catch {
          // Injection fallback failed
        }
      }

      if (!response || !response.supported) {
        setPageState({
          loading: false,
          supported: false,
          error: 'Open a problem on LeetCode, Codeforces, AtCoder, CSES, or CodeChef.',
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
          error: 'No solution code found in editor. Type or paste your code in the editor.',
        });
        return;
      }

      setPageState({
        loading: false,
        supported: true,
        platformName: response.platformName,
        editorAvailable: true,
      });

      const analysis: AnalysisResult = await browser.runtime.sendMessage({
        type: 'ANALYZE_REQUEST',
        payload: {
          source: response.source,
          language: response.language || 'cpp',
        },
      });

      setResult(analysis);
    } catch {
      setPageState({
        loading: false,
        supported: false,
        error: 'Please refresh the problem page and try again.',
      });
    }
  }

  useEffect(() => {
    analyzeTab();
  }, []);

  function handleCopy() {
    if (!result) return;
    const algoStr = result.algorithms.length > 0 ? result.algorithms.join(', ') : 'None';
    const text = `Time Complexity: ${result.tc}\nSpace Complexity: ${result.sc}\nAlgorithm: ${algoStr}`;
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  }

  return (
    <div style={{
      display: 'flex',
      flexDirection: 'column',
      minHeight: '190px',
      background: '#ffffff',
      color: '#111827',
      fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif'
    }}>
      {/* Header */}
      <div style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        padding: '10px 14px',
        borderBottom: '1px solid #e5e7eb',
        background: '#f9fafb'
      }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <Kite0Badge />
          <div style={{ display: 'flex', alignItems: 'baseline', gap: '6px' }}>
            <span style={{ fontSize: '15px', fontWeight: 700, color: '#111827' }}>
              Kite0
            </span>
            {pageState.platformName && (
              <span style={{ fontSize: '12px', color: '#6b7280' }}>
                • {pageState.platformName}
              </span>
            )}
          </div>
        </div>

        <button
          onClick={() => analyzeTab()}
          style={{
            background: '#ffffff',
            border: '1px solid #d1d5db',
            borderRadius: '5px',
            padding: '3px 9px',
            color: '#374151',
            cursor: 'pointer',
            fontSize: '12px',
            fontWeight: 500,
            transition: 'background 0.1s'
          }}
          onMouseEnter={(e) => {
            (e.currentTarget as HTMLElement).style.background = '#f3f4f6';
          }}
          onMouseLeave={(e) => {
            (e.currentTarget as HTMLElement).style.background = '#ffffff';
          }}
        >
          Scan
        </button>
      </div>

      {/* Body */}
      <div style={{ padding: '14px', flex: 1, display: 'flex', flexDirection: 'column', justifyContent: 'center' }}>
        {pageState.loading && (
          <div style={{ textAlign: 'center', padding: '24px 0', color: '#6b7280', fontSize: '13px' }}>
            Analyzing solution...
          </div>
        )}

        {!pageState.loading && pageState.contestMode && (
          <div style={{
            background: '#fffbeb',
            border: '1px solid #fde68a',
            borderRadius: '6px',
            padding: '12px',
            color: '#92400e',
            fontSize: '13px',
            lineHeight: 1.4
          }}>
            <strong>Contest Mode Active</strong>
            <div style={{ marginTop: '4px', fontSize: '12px' }}>
              Live contest session detected. Analysis is paused for fair play.
            </div>
          </div>
        )}

        {!pageState.loading && !pageState.contestMode && pageState.error && (
          <div style={{
            background: '#f9fafb',
            border: '1px solid #e5e7eb',
            borderRadius: '6px',
            padding: '14px',
            textAlign: 'center',
            color: '#4b5563',
            fontSize: '13px',
            lineHeight: 1.5
          }}>
            {pageState.error}
          </div>
        )}

        {!pageState.loading && !pageState.contestMode && result && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
            {/* Complexity Metrics Table */}
            <div style={{
              display: 'grid',
              gridTemplateColumns: '1fr 1fr',
              gap: '8px',
            }}>
              <div style={{
                background: '#f9fafb',
                border: '1px solid #e5e7eb',
                borderRadius: '6px',
                padding: '10px 12px'
              }}>
                <div style={{ fontSize: '11px', fontWeight: 600, color: '#6b7280', textTransform: 'uppercase' }}>
                  Time
                </div>
                <div style={{
                  fontFamily: MONO_FONT,
                  fontSize: '18px',
                  fontWeight: 700,
                  color: '#111827',
                  marginTop: '2px'
                }}>
                  {result.tc}
                </div>
              </div>

              <div style={{
                background: '#f9fafb',
                border: '1px solid #e5e7eb',
                borderRadius: '6px',
                padding: '10px 12px'
              }}>
                <div style={{ fontSize: '11px', fontWeight: 600, color: '#6b7280', textTransform: 'uppercase' }}>
                  Space
                </div>
                <div style={{
                  fontFamily: MONO_FONT,
                  fontSize: '18px',
                  fontWeight: 700,
                  color: '#111827',
                  marginTop: '2px'
                }}>
                  {result.sc}
                </div>
              </div>
            </div>

            {/* Algorithm Detected */}
            <div style={{
              background: '#f9fafb',
              border: '1px solid #e5e7eb',
              borderRadius: '6px',
              padding: '10px 12px',
              display: 'flex',
              alignItems: 'baseline',
              justifyContent: 'space-between'
            }}>
              <span style={{ fontSize: '11px', fontWeight: 600, color: '#6b7280', textTransform: 'uppercase' }}>
                Algorithm
              </span>
              <span style={{
                fontFamily: MONO_FONT,
                fontSize: '13px',
                fontWeight: 600,
                color: result.algorithms.length > 0 ? '#15803d' : '#6b7280'
              }}>
                {result.algorithms.length > 0 ? result.algorithms.join(', ') : 'None detected'}
              </span>
            </div>

            {/* Actions */}
            <div style={{ display: 'flex', justifyContent: 'flex-end', marginTop: '2px' }}>
              <button
                onClick={handleCopy}
                style={{
                  background: '#ffffff',
                  border: '1px solid #d1d5db',
                  borderRadius: '5px',
                  padding: '5px 12px',
                  fontSize: '12px',
                  fontWeight: 500,
                  color: '#374151',
                  cursor: 'pointer'
                }}
                onMouseEnter={(e) => {
                  (e.currentTarget as HTMLElement).style.background = '#f3f4f6';
                }}
                onMouseLeave={(e) => {
                  (e.currentTarget as HTMLElement).style.background = '#ffffff';
                }}
              >
                {copied ? 'Copied' : 'Copy'}
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Footer */}
      <div style={{
        padding: '6px 14px',
        borderTop: '1px solid #f3f4f6',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        fontSize: '11px',
        color: '#9ca3af'
      }}>
        <span>Local WASM</span>
        <span>v0.1.0 • Offline</span>
      </div>
    </div>
  );
}
