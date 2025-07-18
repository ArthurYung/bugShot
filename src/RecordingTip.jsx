import React, { useEffect, useState } from 'react';

function formatTime(sec) {
  const m = String(Math.floor(sec / 60)).padStart(2, '0');
  const s = String(sec % 60).padStart(2, '0');
  return `${m}:${s}`;
}

export default function RecordingTip({ onStop }) {
  const [seconds, setSeconds] = useState(0);

  useEffect(() => {
    const timer = setInterval(() => setSeconds(s => s + 1), 1000);
    return () => clearInterval(timer);
  }, []);

  return (
    <div className="recording-card">
      <span className="breath-light" />
      <span>正在录制</span>
      <span style={{ marginLeft: 10, fontVariantNumeric: 'tabular-nums', fontWeight: 600 }}>
        {formatTime(seconds)}
      </span>
      <button className="stop-btn" onClick={onStop}>停止</button>
    </div>
  );
}