import React from 'react';

function ProxyConfigBox({ rules, onRulesChange, onSave, saving }) {
  return (
    <div className="proxy-config-box">
      <textarea
        className="proxy-config-textarea"
        placeholder="在此输入代理规则，参考whistle文档：https://wproxy.org/whistle/rules.html"
        value={rules}
        onChange={e => onRulesChange(e.target.value)}
        disabled={saving}
      />
      <button
        className="proxy-config-save-btn"
        onClick={onSave}
        disabled={saving}
      >
        {saving ? '保存中...' : '保存'}
      </button>
    </div>
  );
}

export default ProxyConfigBox; 