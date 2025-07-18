import React, { useState, useEffect } from 'react';
import { invoke } from "@tauri-apps/api/core";

function ConfigManager() {
  const [config, setConfig] = useState(null);
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);

  // 加载配置
  const loadConfig = async () => {
    setLoading(true);
    try {
      const result = await invoke('config_get');
      setConfig(result);
    } catch (error) {
      console.error('加载配置失败:', error);
      alert('加载配置失败: ' + error);
    } finally {
      setLoading(false);
    }
  };

  // 保存配置
  const saveConfig = async (updates) => {
    setSaving(true);
    try {
      const result = await invoke('config_update', { updates });
      setConfig(result);
      alert('配置保存成功！');
    } catch (error) {
      console.error('保存配置失败:', error);
      alert('保存配置失败: ' + error);
    } finally {
      setSaving(false);
    }
  };

  // 重置配置
  const resetConfig = async () => {
    if (!confirm('确定要重置所有配置为默认值吗？')) {
      return;
    }
    
    setSaving(true);
    try {
      const result = await invoke('config_reset');
      setConfig(result);
      alert('配置已重置为默认值！');
    } catch (error) {
      console.error('重置配置失败:', error);
      alert('重置配置失败: ' + error);
    } finally {
      setSaving(false);
    }
  };

  // 更新单个配置项
  const updateConfigItem = (key, value) => {
    if (!config) return;
    
    const updates = { [key]: value };
    saveConfig(updates);
  };

  // 组件加载时获取配置
  useEffect(() => {
    loadConfig();
  }, []);

  if (loading) {
    return <div>加载配置中...</div>;
  }

  if (!config) {
    return <div>配置加载失败</div>;
  }

  return (
    <div className="config-manager" style={{ padding: '20px', maxWidth: '600px' }}>
      <h2>抓包配置</h2>
      
      {/* 抓包配置 */}
      <div className="config-section" style={{ marginBottom: '20px', padding: '15px', border: '1px solid #ddd', borderRadius: '5px' }}>
        <h3>抓包设置</h3>
        
        <div style={{ marginBottom: '15px' }}>
          <label style={{ display: 'flex', alignItems: 'center' }}>
            <input
              type="checkbox"
              checked={config.capture_enabled}
              onChange={(e) => updateConfigItem('capture_enabled', e.target.checked)}
              style={{ marginRight: '10px' }}
            />
            启用抓包功能
          </label>
        </div>
        
        <div style={{ marginBottom: '15px' }}>
          <label>抓包规则: </label>
          <textarea
            value={config.capture_rules}
            onChange={(e) => updateConfigItem('capture_rules', e.target.value)}
            style={{ marginTop: '5px', padding: '10px', width: '100%', height: '120px', border: '1px solid #ccc', borderRadius: '3px' }}
            placeholder="每行一个规则，例如：&#10;log&#10;export http://127.0.0.1:3000/whistle-capture"
          />
        </div>
      </div>

      {/* 操作按钮 */}
      <div className="actions" style={{ display: 'flex', gap: '10px' }}>
        <button
          onClick={loadConfig}
          disabled={saving}
          style={{ padding: '10px 15px', backgroundColor: '#6c757d', color: 'white', border: 'none', borderRadius: '5px' }}
        >
          重新加载
        </button>
        
        <button
          onClick={resetConfig}
          disabled={saving}
          style={{ padding: '10px 15px', backgroundColor: '#dc3545', color: 'white', border: 'none', borderRadius: '5px' }}
        >
          重置为默认
        </button>
        
        {saving && <span style={{ color: '#007bff' }}>保存中...</span>}
      </div>

      {/* 配置信息 */}
      <div className="config-info" style={{ marginTop: '20px', padding: '15px', backgroundColor: '#f8f9fa', borderRadius: '5px' }}>
        <h4>配置说明</h4>
        <ul>
          <li><strong>启用抓包功能</strong>：控制是否启用 Whistle 代理抓包</li>
          <li><strong>抓包规则</strong>：Whistle 的代理规则配置，每行一个规则</li>
          <li><strong>常用规则</strong>：log（记录日志）、export（导出数据）</li>
        </ul>
      </div>
    </div>
  );
}

export default ConfigManager; 