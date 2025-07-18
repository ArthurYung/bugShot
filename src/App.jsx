import { useState } from 'react'
import reactLogo from './assets/react.svg'
import './App.css'
import WhistleManager from './WhistleManager'
import ConfigManager from './ConfigManager'

function App() {
  const [greetMsg, setGreetMsg] = useState("")
  const [name, setName] = useState("")
  const [currentTab, setCurrentTab] = useState('whistle') // 'whistle', 'recording', 'config'

  return (
    <div className="container">
      <h1>BugShot - 录屏抓包工具</h1>
      
      {/* 导航标签 */}
      <div className="tabs" style={{ marginBottom: '20px' }}>
        <button 
          onClick={() => setCurrentTab('whistle')}
          style={{ 
            padding: '10px 20px', 
            marginRight: '10px',
            backgroundColor: currentTab === 'whistle' ? '#007bff' : '#f8f9fa',
            color: currentTab === 'whistle' ? 'white' : 'black',
            border: '1px solid #ddd',
            borderRadius: '5px',
            cursor: 'pointer'
          }}
        >
          Whistle 代理
        </button>
        <button 
          onClick={() => setCurrentTab('recording')}
          style={{ 
            padding: '10px 20px',
            marginRight: '10px',
            backgroundColor: currentTab === 'recording' ? '#007bff' : '#f8f9fa',
            color: currentTab === 'recording' ? 'white' : 'black',
            border: '1px solid #ddd',
            borderRadius: '5px',
            cursor: 'pointer'
          }}
        >
          录屏功能
        </button>
        <button 
          onClick={() => setCurrentTab('config')}
          style={{ 
            padding: '10px 20px',
            backgroundColor: currentTab === 'config' ? '#007bff' : '#f8f9fa',
            color: currentTab === 'config' ? 'white' : 'black',
            border: '1px solid #ddd',
            borderRadius: '5px',
            cursor: 'pointer'
          }}
        >
          应用配置
        </button>
      </div>

      {/* 内容区域 */}
      <div className="content">
        {currentTab === 'whistle' ? (
          <WhistleManager />
        ) : currentTab === 'recording' ? (
          <div className="recording-section">
            <h2>录屏功能</h2>
            <p>录屏功能开发中...</p>
            {/* 这里可以添加录屏相关的组件 */}
          </div>
        ) : (
          <ConfigManager />
        )}
      </div>
    </div>
  )
}

export default App
