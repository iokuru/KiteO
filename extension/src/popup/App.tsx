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

function Kite0Logo({ size = 24 }: { size?: number }) {
  return (
    <svg width={size} height={size} viewBox="0 0 64 64" fill="none" style={{ flexShrink: 0 }}>
      <rect x="2" y="2" width="60" height="60" rx="14" fill="#0B0F19" stroke="#1E293B" strokeWidth="1.5" />
      <polygon points="32,10 11.5,29.5 32,29.5" fill="#0EA5E9" />
      <polygon points="32,10 52.5,29.5 32,29.5" fill="#38BDF8" />
      <polygon points="32,54 11.5,29.5 32,29.5" fill="#0369A1" />
      <polygon points="32,54 52.5,29.5 32,29.5" fill="#0284C7" />
      <polygon points="32,10 52.5,29.5 32,54 11.5,29.5" stroke="#BAE6FD" strokeWidth="1" strokeLinejoin="round" fill="none" />
      <line x1="32" y1="10" x2="32" y2="54" stroke="#F0F9FF" strokeOpacity="0.5" strokeWidth="1" />
      <line x1="11.5" y1="29.5" x2="52.5" y2="29.5" stroke="#F0F9FF" strokeOpacity="0.5" strokeWidth="1" />
      <ellipse cx="32" cy="29.5" rx="11" ry="13.5" fill="#0B0F19" fillOpacity="0.9" />
      <ellipse cx="32" cy="29.5" rx="9" ry="11.5" stroke="#FFFFFF" strokeWidth="2.5" fill="none" />
      <line x1="38" y1="21.5" x2="26" y2="37.5" stroke="#FFFFFF" strokeWidth="2" strokeLinecap="round" />
    </svg>
  );
}

function RefreshIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
      <path d="M3 3v5h5" />
      <path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" />
      <path d="M16 21h5v-5" />
    </svg>
  );
}

function ShieldIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#f59e0b" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
    </svg>
  );
}

function CopyIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
      <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
    </svg>
  );
}

function CheckIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
      <polyline points="20 6 9 17 4 12" />
    </svg>
  );
}

const MONO_FONT = "ui-monospace, 'Cascadia Code', 'JetBrains Mono', 'Fira Code', Menlo, Consolas, monospace";

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
        console.warn('Direct message failed, attempting script injection...', err);
      }

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
          error: 'Ready on supported platforms: LeetCode, Codeforces, AtCoder, CSES, CodeChef, HackerRank.',
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
          error: 'Editor detected, but no solution code found yet. Enter your solution code in the editor.',
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

      const analysis: AnalysisResult = await browser.runtime.sendMessage({
        type: 'ANALYZE_REQUEST',
        payload: {
          source: response.source,
          language: response.language || 'cpp',
        },
      });

      if (response.constraints && analysis.tc) {
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
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <Kite0Logo size={24} />
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
              fontFamily: MONO_FONT,
              fontWeight: 600,
              padding: '2px 8px',
              borderRadius: '4px',
              background: '#131e33',
              color: '#38bdf8',
              border: '1px solid #1e3a5f'
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
              gap: '5px',
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
            <RefreshIcon />
            <span>Re-scan</span>
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
              Parsing syntax tree...
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
              <ShieldIcon />
              <span>Contest Fair-Play Active</span>
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
              fontSize: '11px',
              color: '#64748b',
              borderTop: '1px solid #1e293b',
              paddingTop: '8px',
              marginTop: '4px'
            }}>
              Supported: LeetCode, Codeforces, AtCoder, CSES, CodeChef, HackerRank.
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
                  fontFamily: MONO_FONT,
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
                  fontFamily: MONO_FONT,
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
                        fontFamily: MONO_FONT,
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
                {copied ? (
                  <>
                    <CheckIcon />
                    <span>Copied to clipboard</span>
                  </>
                ) : (
                  <>
                    <CopyIcon />
                    <span>Copy Analysis</span>
                  </>
                )}
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
        <span>v0.1.0 / 100% Offline</span>
      </div>
    </div>
  );
}
