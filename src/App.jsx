import { useState, useEffect } from 'react'
import './App.css'
import ProxyConfigBox from './ProxyConfigBox'
import { invoke } from "@tauri-apps/api/core";
import { FiPower, FiDownload, FiTrash2, FiPlayCircle, FiFolder } from 'react-icons/fi';

const cardColors = ['#AEE2FF', '#FFD6A5', '#B4F8C8', '#E4C1F9', '#FFF6B7'];

function App() {
  const [proxyOn, setProxyOn] = useState(false)
  const [loadingProxy, setLoadingProxy] = useState(true)
  const [rules, setRules] = useState('')
  const [savingRules, setSavingRules] = useState(false)
  // 假数据
  const recordings = [
    {
      id: 1,
      thumbnail: 'https://via.placeholder.com/120x68?text=Preview',
      time: '2024-06-01 14:23',
      duration: '00:12',
    },
    {
      id: 2,
      thumbnail: 'https://via.placeholder.com/120x68?text=Preview',
      time: '2024-06-01 15:10',
      duration: '00:45',
    },
    {
      id: 3,
      thumbnail: 'https://via.placeholder.com/120x68?text=Preview',
      time: '2024-06-01 16:00',
      duration: '01:10',
    },
  ]

  useEffect(() => {
    const fetchProxy = async () => {
      setLoadingProxy(true)
      try {
        const config = await invoke('config_get')
        setProxyOn(!!config.capture_enabled)
        setRules(config.capture_rules || '')
      } catch (e) {
        setProxyOn(false)
        setRules('')
      } finally {
        setLoadingProxy(false)
      }
    }
    fetchProxy()
  }, [])

  // 代理开关
  const handleProxyChange = async (checked) => {
    setProxyOn(checked)
    setLoadingProxy(true)
    try {
      await invoke('config_update', { updates: { capture_enabled: checked } })
    } catch (e) {
      alert('切换代理失败: ' + e)
      setProxyOn(!checked)
    } finally {
      setLoadingProxy(false)
    }
  }

  // 保存规则
  const handleSaveRules = async () => {
    setSavingRules(true)
    try {
      await invoke('config_update', { updates: { capture_rules: rules } })
      alert('保存成功！')
    } catch (e) {
      alert('保存失败: ' + e)
    } finally {
      setSavingRules(false)
    }
  }

  return (
    <div className="app-row-layout app-ui-modern">
      {/* 左侧操作区 */}
      <div className="left-panel op-panel">
        <div className="op-section">
          <button className="record-btn">
            <FiPlayCircle style={{marginRight: 8, fontSize: 20}} />
            开始录制
          </button>
        </div>
        <div className="op-section">
          <div className="op-section-title">网络请求录制</div>
          <p className="op-section-desc">开启后，会使用whistle代理所有请求，并录制请求内容。</p>
          <p className="op-section-desc">录制内容会保存到本地，可以随时查看和下载。</p>
          <label className="switch">
            <input type="checkbox" checked={proxyOn} disabled={loadingProxy} onChange={e => handleProxyChange(e.target.checked)} />
            <span className="slider round"></span>
          </label>
        </div>
        <div className="op-section">
          <div className="op-section-title">代理规则设置</div>
          <p className="op-section-desc">设置代理规则，可以录制指定域名的请求。</p>
          <p className="op-section-desc">规则格式参考whistle文档：<a href="https://wproxy.org/whistle/rules.html" target="_blank" rel="noopener noreferrer">https://wproxy.org/whistle/rules.html</a></p>
          <ProxyConfigBox
            rules={rules}
            onRulesChange={setRules}
            onSave={handleSaveRules}
            saving={savingRules}
          />
        </div>
      </div>
      {/* 右侧列表区 */}
      <div className="right-panel list-panel">
        <div className="recordings-list modern-list">
          {recordings.map((item, idx) => (
            <div key={item.id} className="recording-item card-style modern-card" >
              <img src={item.thumbnail} alt="preview" className="recording-thumb modern-thumb" />
              <div className="recording-info modern-info">
                <div className="recording-time modern-time">录制于 {item.time}</div>
                <div className="recording-duration modern-duration">时长：{item.duration}</div>
              </div>
              <div className="recording-actions">
                <span className="icon-btn download-btn" title="打开文件夹"><FiFolder style={{fontSize: 18}} /></span>
                <span className="icon-btn delete-btn"><FiTrash2 style={{fontSize: 18}} /></span>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}

export default App
