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
  sourcePreview?: string;
  language?: string;
}

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
      } catch (err) {
        // Content script might not have been injected yet
        console.warn('Direct message failed, attempting script injection...', err);
      }

      // If no response, try injecting content script dynamically
      if (!response && (browser as any).scripting?.executeScript) {
        try {
          await (browser as any).scripting.executeScript({
            target: { tabId: activeTab.id },
            files: ['content.js']
          });
          response = await browser.tabs.sendMessage(activeTab.id, { type: 'EXTRACT_SOURCE' });
        } catch (injectionErr) {
          console.warn('Scripting injection failed:', injectionErr);
        }
      }

      if (!response || !response.supported) {
        setPageState({
          loading: false,
          supported: false,
          error: 'Ready on competitive programming editors: LeetCode, Codeforces, CodeChef, AtCoder, CSES, HackerRank, GeeksforGeeks, etc.',
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
          error: 'Editor detected, but no solution code found yet. Type or paste your solution in the editor.',
        });
        return;
      }

      setPageState({
        loading: false,
        supported: true,
        platformName: response.platformName,
        editorAvailable: true,
        sourcePreview: response.source.slice(0, 80),
        language: response.language,
      });

      // Request deterministic analysis from background worker (WASM)
      const analysis: AnalysisResult = await browser.runtime.sendMessage({
        type: 'ANALYZE_REQUEST',
        payload: {
          source: response.source,
          language: response.language || 'cpp',
        },
      });

      if (response.constraints && analysis.tc) {
        // Optional limits evaluation
        analysis.constraintStatus = response.constraints;
      }

      setResult(analysis);
    } catch (err: any) {
      setPageState({
        loading: false,
        supported: false,
        error: 'Please refresh the problem page or ensure you are in an active editor tab.',
      });
    }
  }

  useEffect(() => {
    analyzeTab();
  }, []);

  function handleCopySummary() {
    if (!result) return;
    const algoStr = result.algorithms.length > 0 ? result.algorithms.join(', ') : 'Unknown';
    const text = `Time Complexity: ${result.tc}\nSpace Complexity: ${result.sc}\nAlgorithm: ${algoStr}`;
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  }

  return (
    <div style={{ display: 'flex', flexDirection: 'column', minHeight: '280px', background: '#0b0f17' }}>
      {/* Top App Bar */}
      <div style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        padding: '12px 16px',
        background: '#0f172a',
        borderBottom: '1px solid #1e293b'
      }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <div style={{
            width: '24px',
            height: '24px',
            borderRadius: '6px',
            background: 'linear-gradient(135deg, #0284c7 0%, #0369a1 100%)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            fontWeight: 800,
            fontSize: '13px',
            color: '#fff',
            fontFamily: "ui-monospace, 'Cascadia Code', 'JetBrains Mono', 'Fira Code', Menlo, Consolas, monospace",
            boxShadow: '0 2px 4px rgba(2, 132, 199, 0.2)'
          }}>
            K0
          </div>
          <div>
            <div style={{ fontSize: '14px', fontWeight: 700, letterSpacing: '-0.01em', color: '#f8fafc' }}>
              Kite0
            </div>
            <div style={{ fontSize: '10px', color: '#64748b', marginTop: '-2px' }}>
              Static Complexity Engine
            </div>
          </div>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
          {pageState.platformName && (
            <span style={{
              fontSize: '11px',
              fontWeight: 600,
              padding: '2px 8px',
              borderRadius: '999px',
              background: '#1e293b',
              color: '#38bdf8',
              border: '1px solid #334155'
            }}>
              {pageState.platformName}
            </span>
          )}
          <button
            onClick={() => analyzeTab()}
            title="Re-analyze current buffer"
            style={{
              background: 'transparent',
              border: '1px solid #334155',
              borderRadius: '6px',
              padding: '4px 8px',
              color: '#94a3b8',
              cursor: 'pointer',
              fontSize: '11px',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              transition: 'all 0.15s ease'
            }}
            onMouseEnter={(e) => {
              (e.currentTarget as HTMLElement).style.color = '#f8fafc';
              (e.currentTarget as HTMLElement).style.borderColor = '#475569';
            }}
            onMouseLeave={(e) => {
              (e.currentTarget as HTMLElement).style.color = '#94a3b8';
              (e.currentTarget as HTMLElement).style.borderColor = '#334155';
            }}
          >
            ↻ Re-scan
          </button>
        </div>
      </div>

      {/* Main Content Area */}
      <div style={{ padding: '16px', flex: 1, display: 'flex', flexDirection: 'column', gap: '12px' }}>
        {pageState.loading && (
          <div style={{
            padding: '40px 16px',
            textAlign: 'center',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            gap: '12px'
          }}>
            <div style={{
              width: '24px',
              height: '24px',
              border: '2px solid #1e293b',
              borderTopColor: '#38bdf8',
              borderRadius: '50%',
              animation: 'spin 0.8s linear infinite'
            }} />
            <div style={{ fontSize: '13px', color: '#94a3b8' }}>
              Inspecting editor syntax tree...
            </div>
            <style>{`@keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }`}</style>
          </div>
        )}

        {!pageState.loading && pageState.contestMode && (
          <div style={{
            background: 'rgba(245, 158, 11, 0.08)',
            border: '1px solid rgba(245, 158, 11, 0.3)',
            borderRadius: '8px',
            padding: '14px',
            color: '#fef3c7'
          }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px', fontWeight: 600, fontSize: '13px', color: '#fbbf24' }}>
              <span>🛡️</span> Contest Fair-Play Active
            </div>
            <p style={{ margin: '6px 0 0 0', fontSize: '12px', lineHeight: '1.4', color: '#d1d5db' }}>
              Active live contest detected. Kite0 is paused to strictly comply with platform contest integrity standards.
            </p>
          </div>
        )}

        {!pageState.loading && !pageState.contestMode && pageState.error && (
          <div style={{
            background: '#131b2e',
            border: '1px solid #1e293b',
            borderRadius: '8px',
            padding: '16px',
            display: 'flex',
            flexDirection: 'column',
            gap: '10px'
          }}>
            <div style={{ fontSize: '13px', fontWeight: 600, color: '#f8fafc' }}>
              Ready for Code Analysis
            </div>
            <p style={{ margin: 0, fontSize: '12px', color: '#94a3b8', lineHeight: '1.5' }}>
              {pageState.error}
            </p>
            <div style={{
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              fontSize: '11px',
              color: '#64748b',
              borderTop: '1px solid #1e293b',
              paddingTop: '8px',
              marginTop: '4px'
            }}>
              <span>💡</span> Open any LeetCode, Codeforces, or CodeChef problem editor.
            </div>
          </div>
        )}

        {!pageState.loading && !pageState.contestMode && result && (
          <>
            {/* Complexity Cards Grid */}
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '10px' }}>
              {/* Time Complexity Card */}
              <div style={{
                background: '#131b2e',
                border: '1px solid #1e293b',
                borderRadius: '8px',
                padding: '12px',
                position: 'relative',
                overflow: 'hidden'
              }}>
                <div style={{
                  position: 'absolute',
                  top: 0,
                  left: 0,
                  right: 0,
                  height: '2px',
                  background: '#0284c7'
                }} />
                <div style={{
                  fontSize: '10px',
                  fontWeight: 700,
                  color: '#64748b',
                  letterSpacing: '0.06em',
                  textTransform: 'uppercase',
                  marginBottom: '4px'
                }}>
                  TIME COMPLEXITY
                </div>
                <div style={{
                  fontFamily: "ui-monospace, 'Cascadia Code', 'JetBrains Mono', 'Fira Code', Menlo, Consolas, monospace",
                  fontSize: '18px',
                  fontWeight: 700,
                  color: '#38bdf8'
                }}>
                  {result.tc}
                </div>
              </div>

              {/* Space Complexity Card */}
              <div style={{
                background: '#131b2e',
                border: '1px solid #1e293b',
                borderRadius: '8px',
                padding: '12px',
                position: 'relative',
                overflow: 'hidden'
              }}>
                <div style={{
                  position: 'absolute',
                  top: 0,
                  left: 0,
                  right: 0,
                  height: '2px',
                  background: '#10b981'
                }} />
                <div style={{
                  fontSize: '10px',
                  fontWeight: 700,
                  color: '#64748b',
                  letterSpacing: '0.06em',
                  textTransform: 'uppercase',
                  marginBottom: '4px'
                }}>
                  SPACE COMPLEXITY
                </div>
                <div style={{
                  fontFamily: "ui-monospace, 'Cascadia Code', 'JetBrains Mono', 'Fira Code', Menlo, Consolas, monospace",
                  fontSize: '18px',
                  fontWeight: 700,
                  color: '#34d399'
                }}>
                  {result.sc}
                </div>
              </div>
            </div>

            {/* Algorithm Classification Card */}
            <div style={{
              background: '#131b2e',
              border: '1px solid #1e293b',
              borderRadius: '8px',
              padding: '12px',
              position: 'relative',
              overflow: 'hidden'
            }}>
              <div style={{
                position: 'absolute',
                top: 0,
                left: 0,
                right: 0,
                height: '2px',
                background: '#eab308'
              }} />
              <div style={{
                fontSize: '10px',
                fontWeight: 700,
                color: '#64748b',
                letterSpacing: '0.06em',
                textTransform: 'uppercase',
                marginBottom: '8px'
              }}>
                CLASSIFIED ALGORITHMS
              </div>

              <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px' }}>
                {result.algorithms.length > 0 ? (
                  result.algorithms.map((algo) => (
                    <span
                      key={algo}
                      style={{
                        background: 'rgba(234, 179, 8, 0.1)',
                        border: '1px solid rgba(234, 179, 8, 0.3)',
                        color: '#fde047',
                        fontSize: '12px',
                        fontWeight: 600,
                        padding: '3px 8px',
                        borderRadius: '4px'
                      }}
                    >
                      {algo}
                    </span>
                  ))
                ) : (
                  <span style={{ fontSize: '13px', color: '#94a3b8', fontStyle: 'italic' }}>
                    No canonical pattern detected (Unknown)
                  </span>
                )}
              </div>
            </div>

            {/* Copy Button */}
            <div style={{ display: 'flex', justifyContent: 'flex-end', marginTop: '4px' }}>
              <button
                onClick={handleCopySummary}
                style={{
                  background: copied ? '#065f46' : '#1e293b',
                  color: copied ? '#6ee7b7' : '#cbd5e1',
                  border: '1px solid',
                  borderColor: copied ? '#059669' : '#334155',
                  borderRadius: '6px',
                  padding: '6px 12px',
                  fontSize: '12px',
                  fontWeight: 500,
                  cursor: 'pointer',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  transition: 'all 0.15s ease'
                }}
              >
                {copied ? '✓ Copied to clipboard' : '📋 Copy Analysis'}
              </button>
            </div>
          </>
        )}
      </div>

      {/* Footer Status Bar */}
      <div style={{
        padding: '8px 16px',
        background: '#090d16',
        borderTop: '1px solid #1e293b',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        fontSize: '11px',
        color: '#64748b'
      }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
          <span style={{
            display: 'inline-block',
            width: '6px',
            height: '6px',
            borderRadius: '50%',
            background: pageState.supported ? '#10b981' : '#f59e0b'
          }} />
          <span>{pageState.supported ? 'Online (Local WASM)' : 'Standby'}</span>
        </div>
        <span>v0.1.0 • 100% Offline</span>
      </div>
    </div>
  );
}
