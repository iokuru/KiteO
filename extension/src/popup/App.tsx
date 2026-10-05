import { useState } from 'preact/hooks';
import type { AnalysisResult } from '../shared/types';

export function App() {
  const [result, setResult] = useState<AnalysisResult>({
    tc: 'O((n + q) log n)',
    sc: 'O(n + q)',
    algorithms: ['Segment Tree'],
  });

  return (
    <div style={{ padding: '16px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', borderBottom: '1px solid #334155', paddingBottom: '8px' }}>
        <span style={{ fontWeight: 700, fontSize: '15px', letterSpacing: '0.05em' }}>KiteO</span>
        <span style={{ fontSize: '11px', color: '#94a3b8' }}>Local CP Analyzer</span>
      </div>

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
    </div>
  );
}
