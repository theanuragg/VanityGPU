'use client';

import { useState } from 'react';

export default function Home() {
  const [formData, setFormData] = useState({
    solana_pubkey: '',
    desired_prefix: '',
    desired_suffix: '',
    result_db_url: '',
    result_table: 'vanity_results',
    webhook_url: '',
  });

  const [status, setStatus] = useState<null | 'submitting' | 'success' | 'error'>(null);
  const [jobId, setJobId] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setStatus('submitting');

    try {
      const response = await fetch('http://127.0.0.1:8080/jobs', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          ...formData,
          desired_suffix: formData.desired_suffix || null,
        }),
      });

      if (!response.ok) throw new Error('Failed to submit job');

      const data = await response.json();
      setJobId(data.job_id);
      setStatus('success');
    } catch (err) {
      console.error(err);
      setStatus('error');
    }
  };

  return (
    <main className="container">
      <header style={{ textAlign: 'center', marginBottom: '60px' }}>
        <div style={{ display: 'inline-block', padding: '10px 20px', borderRadius: '100px', background: 'rgba(155, 77, 255, 0.1)', color: 'var(--primary)', fontSize: '0.8rem', fontWeight: '600', marginBottom: '20px', border: '1px solid rgba(155, 77, 255, 0.2)' }}>
          SECURE CLUSTER v1.0
        </div>
        <h1 style={{ fontSize: '3.5rem', marginBottom: '10px', background: 'var(--accent-gradient)', WebkitBackgroundClip: 'text', WebkitTextFillColor: 'transparent', lineHeight: '1.2' }}>
          VanityGPU
        </h1>
        <p style={{ color: 'var(--text-secondary)', fontSize: '1.1rem', maxWidth: '600px', margin: '0 auto' }}>
          Generate cryptographically secure Solana vanity addresses using distributed GPU compute. Non-custodial by design.
        </p>
      </header>

      <div className="glass" style={{ maxWidth: '800px', margin: '0 auto', padding: '50px' }}>
        <h2 style={{ marginBottom: '30px', fontSize: '1.5rem', fontWeight: '600' }}>Create New Search Job</h2>

        <form onSubmit={handleSubmit}>
          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '24px' }}>
            <div className="input-group">
              <label className="input-label">Desired Prefix</label>
              <input 
                type="text" 
                placeholder="e.g. SOL" 
                value={formData.desired_prefix}
                onChange={(e) => setFormData({ ...formData, desired_prefix: e.target.value })}
                required
              />
            </div>
            <div className="input-group">
              <label className="input-label">Desired Suffix (Optional)</label>
              <input 
                type="text" 
                placeholder="e.g. 123" 
                value={formData.desired_suffix}
                onChange={(e) => setFormData({ ...formData, desired_suffix: e.target.value })}
              />
            </div>
          </div>

          <div className="input-group">
            <label className="input-label">Your Solana Encryption Pubkey</label>
            <input 
              type="text" 
              placeholder="Base58 Solana Address" 
              value={formData.solana_pubkey}
              onChange={(e) => setFormData({ ...formData, solana_pubkey: e.target.value })}
              required
            />
            <small style={{ color: '#888', marginTop: '8px', display: 'block', fontSize: '0.8rem' }}>
              Used for ECDH key agreement. Your private seed will be encrypted before leaving the worker.
            </small>
          </div>

          <div style={{ margin: '40px 0', position: 'relative' }}>
             <hr style={{ border: 'none', height: '1px', background: 'var(--card-border)' }} />
             <span style={{ position: 'absolute', top: '50%', left: '50%', transform: 'translate(-50%, -50%)', background: '#111', padding: '0 15px', fontSize: '0.7rem', color: '#555', fontWeight: 'bold', letterSpacing: '2px' }}>DELIVERY CONFIG</span>
          </div>

          <div className="input-group">
            <label className="input-label">Result Database URL (Postgres)</label>
            <input 
              type="text" 
              placeholder="postgres://user:pass@host/db" 
              value={formData.result_db_url}
              onChange={(e) => setFormData({ ...formData, result_db_url: e.target.value })}
              required
            />
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '24px' }}>
            <div className="input-group">
              <label className="input-label">Target Table</label>
              <input 
                type="text" 
                value={formData.result_table}
                onChange={(e) => setFormData({ ...formData, result_table: e.target.value })}
                required
              />
            </div>
            <div className="input-group">
              <label className="input-label">Webhook URL (Optional)</label>
              <input 
                type="text" 
                placeholder="https://your-api.com/callback" 
                value={formData.webhook_url}
                onChange={(e) => setFormData({ ...formData, webhook_url: e.target.value })}
              />
            </div>
          </div>

          <button 
            type="submit" 
            className="btn" 
            style={{ width: '100%', marginTop: '30px', padding: '18px' }}
            disabled={status === 'submitting'}
          >
            {status === 'submitting' ? 'Initializing Cluster...' : 'Deploy GPU Task'}
          </button>
        </form>

        {status === 'success' && (
          <div className="glass" style={{ marginTop: '30px', padding: '25px', borderColor: 'var(--secondary)', background: 'rgba(0, 242, 255, 0.05)' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '15px' }}>
               <div style={{ width: '12px', height: '12px', borderRadius: '50%', background: 'var(--secondary)', boxShadow: '0 0 10px var(--secondary)' }}></div>
               <p style={{ color: 'var(--secondary)', fontWeight: '600', fontSize: '1.1rem' }}>Success: Job Enqueued</p>
            </div>
            <p style={{ marginTop: '12px', fontSize: '0.9rem', color: '#ccc' }}>
               Your task has been broadcast to the worker pool. Results will be delivered to your database.
            </p>
            <div style={{ marginTop: '15px', padding: '10px 15px', background: 'rgba(0,0,0,0.3)', borderRadius: '8px', border: '1px solid var(--card-border)' }}>
               <span style={{ fontSize: '0.75rem', color: '#666', fontWeight: 'bold' }}>TRACKING ID:</span>
               <code style={{ fontSize: '0.85rem', color: 'white', marginLeft: '10px' }}>{jobId}</code>
            </div>
          </div>
        )}

        {status === 'error' && (
          <div className="glass" style={{ marginTop: '30px', padding: '25px', borderColor: '#ff4d4d', background: 'rgba(255, 77, 77, 0.05)' }}>
            <p style={{ color: '#ff4d4d', fontWeight: '600' }}>Transmission Failure</p>
            <p style={{ fontSize: '0.9rem', color: '#ff8080', marginTop: '5px' }}>
               Could not reach the local API at 127.0.0.1:8080.
            </p>
          </div>
        )}
      </div>

      <div style={{ display: 'flex', justifyContent: 'center', gap: '40px', marginTop: '60px', color: '#444' }}>
         <div style={{ textAlign: 'center' }}>
            <div style={{ fontSize: '1.2rem', fontWeight: '600', color: '#666' }}>15.4 TFLOPS</div>
            <div style={{ fontSize: '0.7rem', letterSpacing: '1px' }}>CLUSTER POWER</div>
         </div>
         <div style={{ textAlign: 'center' }}>
            <div style={{ fontSize: '1.2rem', fontWeight: '600', color: '#666' }}>X25519</div>
            <div style={{ fontSize: '0.7rem', letterSpacing: '1px' }}>ENCRYPTION</div>
         </div>
         <div style={{ textAlign: 'center' }}>
            <div style={{ fontSize: '1.2rem', fontWeight: '600', color: '#666' }}>ZEROIZED</div>
            <div style={{ fontSize: '0.7rem', letterSpacing: '1px' }}>MEMORY SAFETY</div>
         </div>
      </div>

      <footer style={{ marginTop: '60px', textAlign: 'center', color: '#333', fontSize: '0.75rem', letterSpacing: '1px' }}>
        &copy; 2026 VANITYGPU PROTOCOL. ALL RIGHTS RESERVED.
      </footer>
    </main>
  );
}
