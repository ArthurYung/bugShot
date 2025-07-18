import React, { useState, useEffect } from 'react';
import { invoke } from "@tauri-apps/api/core";

function WhistleManager() {
  const [environment, setEnvironment] = useState(null);
  const [status, setStatus] = useState(null);
  const [loading, setLoading] = useState(false);
  const [config, setConfig] = useState({
    port: 8899,
    rules: ['log', 'export http://127.0.0.1:3000/whistle-capture'],
    storage: null
  });

  // 检查环境
  const checkEnvironment = async () => {
    try {
      const result = await invoke('whistle_check_environment');
      setEnvironment(result);
    } catch (error) {
      console.error('检查环境失败:', error);
    }
  };

  // 获取状态
  const getStatus = async () => {
    try {
      const result = await invoke('whistle_get_status');
      setStatus(result);
    } catch (error) {
      console.error('获取状态失败:', error);
    }
  };

  // 安装 Whistle
  const installWhistle = async () => {
    setLoading(true);
    try {
      await invoke('whistle_install');
      alert('Whistle 安装成功！');
      await checkEnvironment();
    } catch (error) {
      alert('安装失败: ' + error);
    } finally {
      setLoading(false);
    }
  };

  // 启动 Whistle
  const startWhistle = async () => {
    setLoading(true);
    try {
      const result = await invoke('whistle_start', { config });
      alert(result.message);
      await getStatus();
      await checkEnvironment();
    } catch (error) {
      alert('启动失败: ' + error);
    } finally {
      setLoading(false);
    }
  };

  // 停止 Whistle
  const stopWhistle = async () => {
    setLoading(true);
    try {
      const result = await invoke('whistle_stop');
      alert(result.message);
      await getStatus();
      await checkEnvironment();
    } catch (error) {
      alert('停止失败: ' + error);
    } finally {
      setLoading(false);
    }
  };

  // 打开 Web UI
  const openWebUI = async () => {
    try {
      await invoke('whistle_open_web_ui');
    } catch (error) {
      alert('打开失败: ' + error);
    }
  };

  // 组件加载时检查环境
  useEffect(() => {
    checkEnvironment();
    getStatus();
  }, []);

  return (
    <div className="whistle-manager" style={{ padding: '20px', maxWidth: '800px' }}>
      <h2>Whistle 代理管理</h2>
      
      {/* 环境检查 */}
      <div className="environment-section" style={{ marginBottom: '20px', padding: '15px', border: '1px solid #ddd', borderRadius: '5px' }}>
        <h3>环境检查</h3>
        {environment ? (
          <div>
            <p>Node.js: {environment.nodejs_installed ? '✅ 已安装' : '❌ 未安装'}</p>
            <p>Whistle: {environment.whistle_installed ? '✅ 已安装' : '❌ 未安装'}</p>
            <p>运行状态: {environment.whistle_running ? '✅ 运行中' : '❌ 未运行'}</p>
            {environment.whistle_url && (
              <p>管理地址: <a href={environment.whistle_url} target="_blank" rel="noopener noreferrer">{environment.whistle_url}</a></p>
            )}
          </div>
        ) : (
          <p>检查中...</p>
        )}
        <button onClick={checkEnvironment} disabled={loading}>刷新检查</button>
      </div>

      {/* 配置设置 */}
      <div className="config-section" style={{ marginBottom: '20px', padding: '15px', border: '1px solid #ddd', borderRadius: '5px' }}>
        <h3>配置设置</h3>
        <div style={{ marginBottom: '10px' }}>
          <label>端口: </label>
          <input
            type="number"
            value={config.port}
            onChange={(e) => setConfig({...config, port: parseInt(e.target.value)})}
            style={{ marginLeft: '10px', padding: '5px' }}
          />
        </div>
        <div style={{ marginBottom: '10px' }}>
          <label>规则: </label>
          <textarea
            value={config.rules.join('\n')}
            onChange={(e) => setConfig({...config, rules: e.target.value.split('\n').filter(r => r.trim())})}
            style={{ marginLeft: '10px', padding: '5px', width: '400px', height: '60px' }}
            placeholder="每行一个规则"
          />
        </div>
      </div>

      {/* 操作按钮 */}
      <div className="actions-section" style={{ marginBottom: '20px' }}>
        <h3>操作</h3>
        <div style={{ display: 'flex', gap: '10px', flexWrap: 'wrap' }}>
          {!environment?.whistle_installed && (
            <button 
              onClick={installWhistle} 
              disabled={loading || !environment?.nodejs_installed}
              style={{ padding: '10px 15px', backgroundColor: '#007bff', color: 'white', border: 'none', borderRadius: '5px' }}
            >
              {loading ? '安装中...' : '安装 Whistle'}
            </button>
          )}
          
          {environment?.whistle_installed && !status?.running && (
            <button 
              onClick={startWhistle} 
              disabled={loading}
              style={{ padding: '10px 15px', backgroundColor: '#28a745', color: 'white', border: 'none', borderRadius: '5px' }}
            >
              {loading ? '启动中...' : '启动 Whistle'}
            </button>
          )}
          
          {status?.running && (
            <>
              <button 
                onClick={stopWhistle} 
                disabled={loading}
                style={{ padding: '10px 15px', backgroundColor: '#dc3545', color: 'white', border: 'none', borderRadius: '5px' }}
              >
                {loading ? '停止中...' : '停止 Whistle'}
              </button>
              
              <button 
                onClick={openWebUI} 
                disabled={loading}
                style={{ padding: '10px 15px', backgroundColor: '#17a2b8', color: 'white', border: 'none', borderRadius: '5px' }}
              >
                打开管理界面
              </button>
            </>
          )}
        </div>
      </div>

      {/* 状态显示 */}
      <div className="status-section" style={{ padding: '15px', border: '1px solid #ddd', borderRadius: '5px' }}>
        <h3>当前状态</h3>
        {status ? (
          <div>
            <p>运行状态: {status.running ? '✅ 运行中' : '❌ 未运行'}</p>
            {status.url && (
              <p>管理地址: <a href={status.url} target="_blank" rel="noopener noreferrer">{status.url}</a></p>
            )}
          </div>
        ) : (
          <p>获取状态中...</p>
        )}
        <button onClick={getStatus} disabled={loading}>刷新状态</button>
      </div>

      {/* 使用说明 */}
      <div className="instructions" style={{ marginTop: '20px', padding: '15px', backgroundColor: '#f8f9fa', borderRadius: '5px' }}>
        <h3>使用说明</h3>
        <ol>
          <li>确保已安装 Node.js</li>
          <li>点击"安装 Whistle"（首次使用）</li>
          <li>配置端口和规则（可选）</li>
          <li>点击"启动 Whistle"</li>
          <li>系统会自动设置代理到 127.0.0.1:8899</li>
          <li>点击"打开管理界面"查看抓包数据</li>
          <li>使用完毕后点击"停止 Whistle"清除代理</li>
        </ol>
        <p><strong>注意：</strong>首次使用 HTTPS 抓包需要安装 Whistle 的根证书，访问 <a href="http://127.0.0.1:8899" target="_blank" rel="noopener noreferrer">http://127.0.0.1:8899</a> 按提示安装。</p>
      </div>
    </div>
  );
}

export default WhistleManager; 